use anyhow::{Context, Result};
use eventstore::{
    AppendToStreamOptions, Client as EventDb, EventData, ExpectedRevision, ReadStreamOptions,
    StreamPosition,
};
use redis::{Client as CacheDb, FromRedisValue};
use redis::{Commands, ToRedisArgs};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Metadata {
    #[serde(rename = "$correlationId")]
    correlation_id: Uuid,
    #[serde(rename = "$causationId")]
    causation_id: Uuid,
}

pub trait Command: Serialize + DeserializeOwned {
    fn command_name(&self) -> &str;

    fn to_event_data(&self) -> (EventData, Uuid) {
        let id = Uuid::new_v4();
        let mut event_data = EventData::json(self.command_name(), self).unwrap();
        event_data = event_data.id(id);

        event_data = event_data
            .metadata_as_json(Metadata {
                correlation_id: id,
                causation_id: id,
            })
            .unwrap();

        (event_data, id)
    }
}

pub trait Event: Serialize + DeserializeOwned {
    fn event_name(&self) -> &str;

    fn to_event_data(&self, command: Uuid, previous: Uuid) -> (EventData, Uuid) {
        let id = Uuid::new_v4();
        let mut event_data = EventData::json(self.event_name(), self).unwrap();
        event_data = event_data.id(id);

        event_data = event_data
            .metadata_as_json(Metadata {
                correlation_id: command,
                causation_id: previous,
            })
            .unwrap();

        (event_data, id)
    }
}

pub trait ModelEvent: FromRedisValue + Default + ToRedisArgs {
    type Event: Event;
    type Command: Command;

    fn play_event(&mut self, event: &Self::Event);

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>>;

    fn get_position(&self) -> u64;
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

pub struct ModelRepository {
    event_db: EventDb,
    cache_db: CacheDb,
}

impl ModelRepository {
    pub fn new(event_db: EventDb, cache_db: CacheDb) -> Self {
        Self { event_db, cache_db }
    }
    pub async fn get_model<C, E, T>(&self, key: &ModelKey) -> Result<T>
    where
        C: Command,
        E: Event,
        T: ModelEvent<Command = C, Event = E>,
    {
        let mut cache_connection = self
            .cache_db
            .get_connection()
            .context("connect to cache db")?;
        let mut value: T = cache_connection.get(key.format()).unwrap_or_default();

        let options = ReadStreamOptions::default();
        let options = options.position(StreamPosition::Position(value.get_position()));

        let mut stream = self
            .event_db
            .read_stream(key.format(), &options)
            .await
            .context("connect to event db")?;

        let mut has_changed = false;

        while let Ok(Some(json_event)) = stream.next().await {
            has_changed = true;

            let event = json_event
                .get_original_event()
                .as_json::<E>()
                .context("decode event")?;
            value.play_event(&event);
        }

        if has_changed {
            cache_connection
                .set(key.format(), &value)
                .context("set cache value")?;
        }

        Ok(value)
    }

    pub async fn add_command<C, E, T>(&self, key: &ModelKey, command: C) -> Result<T>
    where
        C: Command,
        E: Event,
        T: ModelEvent<Command = C, Event = E>,
    {
        let mut model: T = self.get_model(key).await.context("adding command")?;

        let events = model.try_command(&command).context("try command")?;

        let options = AppendToStreamOptions::default()
            .expected_revision(ExpectedRevision::Exact(model.get_position()));

        let (command_data, command_uuid) = command.to_event_data();

        let mut events_data = vec![command_data];

        let mut previous_uuid = command_uuid;

        for event in &events {
            let (event_data, uuid) = event.to_event_data(command_uuid, previous_uuid);
            events_data.push(event_data);
            previous_uuid = uuid;
        }

        self.event_db
            .append_to_stream(key.format(), &options, events_data)
            .await
            .context("write events")?;

        for event in &events {
            model.play_event(event);
        }

        Ok(model)
    }
}
