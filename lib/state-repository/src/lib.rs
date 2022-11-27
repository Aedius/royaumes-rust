use anyhow::{anyhow, Context, Result};
use eventstore::{
    AppendToStreamOptions, Client as EventDb, Error, EventData, ExpectedRevision,
    ReadStreamOptions, StreamPosition,
};
use redis::Client as CacheDb;
use redis::Commands;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Metadata {
    #[serde(skip_serializing)]
    id: Option<Uuid>,
    #[serde(rename = "$correlationId")]
    correlation_id: Uuid,
    #[serde(rename = "$causationId")]
    causation_id: Uuid,
    #[serde(rename = "is_event")]
    is_event: bool,
}

impl Metadata {
    pub fn correlation_id(&self) -> Uuid {
        self.correlation_id
    }
    pub fn causation_id(&self) -> Uuid {
        self.causation_id
    }
    pub fn set_id(&mut self, id: Option<Uuid>) {
        self.id = id;
    }
    pub fn id(&self) -> Option<Uuid> {
        self.id
    }
}

pub fn to_command_data<T: Command>(
    command: &T,
    previous_metadata: Option<Metadata>,
) -> (EventData, Metadata) {
    let id = Uuid::new_v4();
    let mut event_data = EventData::json(
        format!("{}.{}", T::name_prefix(), command.command_name()),
        command,
    )
    .unwrap();
    event_data = event_data.id(id);

    let metadata = match previous_metadata {
        None => Metadata {
            id: Some(id),
            correlation_id: id,
            causation_id: id,
            is_event: false,
        },
        Some(previous) => Metadata {
            id: Some(id),
            correlation_id: previous.correlation_id,
            causation_id: match previous.id {
                None => id,
                Some(p) => p,
            },
            is_event: false,
        },
    };

    event_data = event_data.metadata_as_json(&metadata).unwrap();

    (event_data, metadata)
}

pub fn to_event_data<T: Event>(event: &T, previous: Metadata) -> (EventData, Metadata) {
    let id = Uuid::new_v4();
    let mut event_data = EventData::json(
        format!("{}.{}", T::name_prefix(), event.event_name()),
        event,
    )
    .unwrap();
    event_data = event_data.id(id);

    let metadata = Metadata {
        id: Some(id),
        correlation_id: previous.correlation_id,
        causation_id: match previous.id {
            None => id,
            Some(uuid) => uuid,
        },
        is_event: true,
    };

    event_data = event_data.metadata_as_json(&metadata).unwrap();

    (event_data, metadata)
}

#[derive(Debug)]
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
        format!("{}.{}", self.stream_name.replace('.', "_"), self.stream_id)
    }
}

impl From<String> for ModelKey {
    fn from(value: String) -> Self {
        let mut split = value.split('.');
        let stream_name = split.next().unwrap_or_default();
        let stream_id = split.collect();
        ModelKey {
            stream_name: stream_name.to_string(),
            stream_id,
        }
    }
}

#[derive(Clone)]
pub struct StateRepository {
    event_db: EventDb,
    cache_db: CacheDb,
}

impl StateRepository {
    pub fn new(event_db: EventDb, cache_db: CacheDb) -> Self {
        Self { event_db, cache_db }
    }
    pub async fn get_model<T>(&self, key: &ModelKey) -> Result<T>
    where
        T: State,
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
                    .as_json::<T::Event>()
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

    pub async fn add_command<T>(
        &self,
        key: &ModelKey,
        command: T::Command,
        previous_metadata: Option<Metadata>,
    ) -> Result<T>
    where
        T: State,
    {
        let mut model: T;
        let events: Vec<T::Event>;

        loop {
            let (l_model, l_events, retry) = self
                .try_append(key, &command, previous_metadata.clone())
                .await?;
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

    async fn try_append<T>(
        &self,
        key: &ModelKey,
        command: &T::Command,
        previous_metadata: Option<Metadata>,
    ) -> Result<(T, Vec<T::Event>, bool)>
    where
        T: State,
    {
        let model: T = self.get_model(key).await.context("adding command")?;

        let events = model.try_command(command).context("try command")?;

        let position = model.get_position();
        let options = if position == 0 {
            AppendToStreamOptions::default().expected_revision(ExpectedRevision::NoStream)
        } else {
            AppendToStreamOptions::default().expected_revision(ExpectedRevision::Exact(position))
        };

        let (command_data, metadata) = to_command_data(command, previous_metadata);

        let mut events_data = vec![command_data];

        let mut previous_metadata = metadata;

        for event in &events {
            let (event_data, metadata) = to_event_data(event, previous_metadata);
            events_data.push(event_data);
            previous_metadata = metadata;
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

    pub fn event_db(&self) -> &EventDb {
        &self.event_db
    }
    pub fn cache_db(&self) -> &CacheDb {
        &self.cache_db
    }
}
