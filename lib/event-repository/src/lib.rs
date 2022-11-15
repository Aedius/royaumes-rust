use anyhow::{anyhow, Context, Result};
use eventstore::{
    AppendToStreamOptions, Client as EventDb, Error, EventData, ExpectedRevision,
    ReadStreamOptions, StreamPosition,
};
use redis::Client as CacheDb;
use redis::Commands;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Metadata {
    #[serde(rename = "$correlationId")]
    correlation_id: Uuid,
    #[serde(rename = "$causationId")]
    causation_id: Uuid,
    #[serde(rename = "is_event")]
    is_event: bool,
}

pub trait Command: Serialize + DeserializeOwned {
    fn name_prefix() -> &'static str;
    fn command_name(&self) -> &str;

    fn to_event_data(&self) -> (EventData, Uuid) {
        let id = Uuid::new_v4();
        let mut event_data = EventData::json(
            format!("{}.{}", Self::name_prefix(), self.command_name()),
            self,
        )
        .unwrap();
        event_data = event_data.id(id);

        event_data = event_data
            .metadata_as_json(Metadata {
                correlation_id: id,
                causation_id: id,
                is_event: false,
            })
            .unwrap();

        (event_data, id)
    }
}

pub trait Event: Serialize + DeserializeOwned {
    fn name_prefix() -> &'static str;
    fn event_name(&self) -> &str;

    fn to_event_data(&self, command: Uuid, previous: Uuid) -> (EventData, Uuid) {
        let id = Uuid::new_v4();
        let mut event_data = EventData::json(
            format!("{}.{}", Self::name_prefix(), self.event_name()),
            self,
        )
        .unwrap();
        event_data = event_data.id(id);

        event_data = event_data
            .metadata_as_json(Metadata {
                correlation_id: command,
                causation_id: previous,
                is_event: true,
            })
            .unwrap();

        (event_data, id)
    }
}

pub trait State: Default + Serialize + DeserializeOwned + Debug {
    type Event: Event;
    type Command: Command;

    fn play_event(&mut self, event: &Self::Event);

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>>;

    fn get_position(&self) -> u64;

    fn set_position(&mut self, pos: u64);

    fn state_cache_interval() -> Option<u64>;
}

pub struct ModelKey {
    stream_name: String,
    stream_id: String,
}

impl ModelKey {
    pub fn new(stream_name: String, stream_id: String) -> Self {
        Self {
            stream_name,
            stream_id,
        }
    }

    fn format(&self) -> String {
        format!("{}-{}", self.stream_name, self.stream_id)
    }
}

pub struct StateRepository {
    event_db: EventDb,
    cache_db: CacheDb,
}

impl StateRepository {
    pub fn new(event_db: EventDb, cache_db: CacheDb) -> Self {
        Self { event_db, cache_db }
    }
    pub async fn get_model<C, E, T>(&self, key: &ModelKey) -> Result<T>
    where
        C: Command,
        E: Event,
        T: State<Command = C, Event = E>,
    {
        let mut value: T;
        if T::state_cache_interval().is_some() {
            let mut cache_connection = self
                .cache_db
                .get_connection()
                .context("connect to cache db")?;
            let data: String = cache_connection
                .get(key.format())
                .context("get from cache")
                .unwrap_or_default();

            value = serde_json::from_str(data.as_str()).unwrap_or_default();
        } else {
            value = T::default();
        }

        let options = ReadStreamOptions::default();
        let options = options.position(StreamPosition::Position(value.get_position() + 1));

        let mut stream = self
            .event_db
            .read_stream(key.format(), &options)
            .await
            .context("connect to event db")?;

        let mut nb_change = 0;

        while let Ok(Some(json_event)) = stream.next().await {
            let original_event = json_event.get_original_event();

            let metadata: Metadata = serde_json::from_str(
                std::str::from_utf8(original_event.custom_metadata.as_ref()).unwrap_or_default(),
            )
            .unwrap();

            if metadata.is_event {
                let event = json_event
                    .get_original_event()
                    .as_json::<E>()
                    .context(format!("decode event : {:?}", json_event))?;

                value.play_event(&event);
                nb_change += 1;
            }
            value.set_position(original_event.revision)
        }

        if T::state_cache_interval().is_some() && nb_change > T::state_cache_interval().unwrap() {
            let mut cache_connection = self
                .cache_db
                .get_connection()
                .context("connect to cache db")?;

            cache_connection
                .set(key.format(), serde_json::to_string(&value)?)
                .context("set cache value")?;
        }

        Ok(value)
    }

    pub async fn add_command<C, E, T>(&self, key: &ModelKey, command: C) -> Result<T>
    where
        C: Command,
        E: Event,
        T: State<Command = C, Event = E>,
    {
        let mut model: T;
        let events: Vec<E>;

        loop {
            let (l_model, l_events, retry) = self.try_append(key, &command).await?;
            if retry {
                continue;
            }
            model = l_model;
            events = l_events;

            break;
        }

        for event in &events {
            model.play_event(event);
        }

        Ok(model)
    }

    async fn try_append<C, E, T>(&self, key: &ModelKey, command: &C) -> Result<(T, Vec<E>, bool)>
    where
        C: Command,
        E: Event,
        T: State<Command = C, Event = E>,
    {
        let model: T = self.get_model(key).await.context("adding command")?;

        let events = model.try_command(command).context("try command")?;

        let position = model.get_position();
        let options = if position == 0 {
            AppendToStreamOptions::default().expected_revision(ExpectedRevision::NoStream)
        } else {
            AppendToStreamOptions::default().expected_revision(ExpectedRevision::Exact(position))
        };

        let (command_data, command_uuid) = command.to_event_data();

        let mut events_data = vec![command_data];

        let mut previous_uuid = command_uuid;

        for event in &events {
            let (event_data, uuid) = event.to_event_data(command_uuid, previous_uuid);
            events_data.push(event_data);
            previous_uuid = uuid;
        }

        let appended = self
            .event_db
            .append_to_stream(key.format(), &options, events_data)
            .await;

        let mut retry = false;

        if appended.is_err() {
            let err = appended.unwrap_err();
            match err {
                Error::WrongExpectedVersion { .. } => {
                    retry = true;
                }
                _ => {
                    return Err(anyhow!("error while appending : {:?}", err));
                }
            }
        }

        Ok((model, events, retry))
    }
}
