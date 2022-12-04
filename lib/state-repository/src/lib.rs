pub mod waiter;

use anyhow::{anyhow, Context, Result};
use eventstore::{AppendToStreamOptions, Client as EventDb, Error, EventData, ExpectedRevision, ReadStreamOptions, StreamPosition};
use redis::Client as CacheDb;
use redis::Commands;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State, StateName};
use std::fmt::Debug;
use uuid::Uuid;

const COMMAND_PREFIX :&'static str = "cmd";
const EVENT_PREFIX :&'static str = "evt";

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

    pub fn new(id: Option<Uuid>, correlation_id: Uuid, causation_id: Uuid, is_event: bool) -> Self {
        Self {
            id,
            correlation_id,
            causation_id,
            is_event,
        }
    }
}

#[derive(Clone)]
pub struct EventWithMetadata {
    event_data: EventData,
    metadata: Metadata,
}

impl EventWithMetadata {
    pub fn event_data(&self) -> &EventData {
        &self.event_data
    }
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn full_event_data(&self) -> EventData {
        self.event_data
            .clone()
            .metadata_as_json(self.metadata())
            .unwrap()
    }

    pub fn from_command<C>(
        command: C,
        previous_metadata: Option<&Metadata>,
        state_name: StateName,
    ) -> Self
    where
        C: Command,
    {
        let event_data = EventData::json(
            format!("{}.{}.{}",COMMAND_PREFIX,  state_name, command.command_name()),
            command,
        )
        .unwrap();

        Self::from_event_data(event_data, previous_metadata, false)
    }

    pub fn from_event<E>(event: E, previous_metadata: &Metadata, state_name: StateName) -> Self
    where
        E: Event,
    {
        let event_data =
            EventData::json(format!("{}.{}.{}",EVENT_PREFIX, state_name, event.event_name()), event).unwrap();

        Self::from_event_data(event_data, Some(previous_metadata), true)
    }

    fn from_event_data(
        mut event_data: EventData,
        previous_metadata: Option<&Metadata>,
        is_event: bool,
    ) -> Self {
        let id = Uuid::new_v4();

        event_data = event_data.id(id);

        let metadata = match previous_metadata {
            None => Metadata {
                id: Some(id),
                correlation_id: id,
                causation_id: id,
                is_event,
            },
            Some(previous) => Metadata {
                id: Some(id),
                correlation_id: previous.correlation_id,
                causation_id: match previous.id {
                    None => id,
                    Some(p) => p,
                },
                is_event,
            },
        };

        Self {
            event_data,
            metadata,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
struct StateInformation {
    position: Option<u64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct StateWithInfo<S> {
    info: StateInformation,
    state: S,
}

impl<S> StateWithInfo<S> {
    pub fn state(&self) -> &S {
        &self.state
    }
}

impl StateRepository {
    pub fn new(event_db: EventDb, cache_db: CacheDb) -> Self {
        Self { event_db, cache_db }
    }

    pub async fn get_model<S>(&self, key: &ModelKey) -> Result<StateWithInfo<S>>
    where
        S: State + DeserializeOwned,
    {
        let value: StateWithInfo<S>;
        if S::state_cache_interval().is_some() {
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
            value = StateWithInfo::default();
        }

        let mut state = value.state;
        let mut info = value.info;

        let options = ReadStreamOptions::default();
        let options = if let Some(position) = info.position {
            options.position(StreamPosition::Position(position + 1))
        } else {
            options.position(StreamPosition::Start)
        };

        let mut stream = self
            .event_db
            .read_stream(key.format(), &options)
            .await
            .context("connect to event db")?;

        let mut nb_change = 0;

        while let Ok(Some(json_event)) = stream.next().await {

            let original_event = json_event.get_original_event();

            let metadata: Metadata = serde_json::from_slice(
                original_event.custom_metadata.as_ref()).unwrap();

            if metadata.is_event {
                let event = original_event
                    .as_json::<S::Event>()
                    .context(format!("decode event : {:?}", json_event))?;

                state.play_event(&event);
                nb_change += 1;
            }

            info.position = Some(original_event.revision)
        }

        let result = StateWithInfo { info, state };

        if S::state_cache_interval().is_some() && nb_change > S::state_cache_interval().unwrap() {
            let mut cache_connection = self
                .cache_db
                .get_connection()
                .context("connect to cache db")?;

            cache_connection
                .set(key.format(), serde_json::to_string(&result)?)
                .context("set cache value")?;
        }

        Ok(result)
    }

    pub async fn add_command<T>(
        &self,
        key: &ModelKey,
        command: T::Command,
        previous_metadata: Option<&Metadata>,
    ) -> Result<T>
    where
        T: State,
    {
        let mut model: T;
        let events: Vec<T::Event>;

        loop {
            let (l_model, l_events, retry) = self
                .try_append(key, command.clone(), previous_metadata)
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

    async fn try_append<S>(
        &self,
        key: &ModelKey,
        command: S::Command,
        previous_metadata: Option<&Metadata>,
    ) -> Result<(S, Vec<S::Event>, bool)>
    where
        S: State,
    {
        let model: StateWithInfo<S> = self.get_model(key).await.context("adding command")?;

        let state = model.state;
        let info = model.info;

        let events = state.try_command(command.clone()).context("try command")?;

        let options = if let Some(position) = info.position {
            AppendToStreamOptions::default().expected_revision(ExpectedRevision::Exact(position))
        } else {
            AppendToStreamOptions::default().expected_revision(ExpectedRevision::NoStream)
        };

        let command_metadata =
            EventWithMetadata::from_command(command, previous_metadata, S::name_prefix());

        let mut events_data = vec![command_metadata.clone()];

        let mut previous_metadata = command_metadata.metadata().to_owned();

        let res_events = events.clone();

        for event in events {
            let event_metadata =
                EventWithMetadata::from_event(event, &previous_metadata, S::name_prefix());

            events_data.push(event_metadata.clone());
            previous_metadata = event_metadata.metadata().to_owned();
        }

        let retry = self
            .try_append_event_data(key, &options, events_data)
            .await?;

        Ok((state, res_events, retry))
    }

    pub async fn try_append_event_data(
        &self,
        key: &ModelKey,
        options: &AppendToStreamOptions,
        events_with_data: Vec<EventWithMetadata>,
    ) -> Result<bool> {
        let events: Vec<EventData> = events_with_data
            .into_iter()
            .map(|e| e.full_event_data())
            .collect();

        let appended = self
            .event_db
            .append_to_stream(key.format(), &options, events)
            .await;

        let mut retry = false;

        if appended.is_err() {
            let err = appended.unwrap_err();
            match err {
                Error::WrongExpectedVersion { expected, current } => {
                    println!("{current} instead of {expected}");

                    retry = true;
                }
                _ => {
                    return Err(anyhow!("error while appending : {:?}", err));
                }
            }
        }
        Ok(retry)
    }

    pub fn event_db(&self) -> &EventDb {
        &self.event_db
    }
    pub fn cache_db(&self) -> &CacheDb {
        &self.cache_db
    }
}
