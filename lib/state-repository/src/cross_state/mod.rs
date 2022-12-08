use crate::metadata::Metadata;
use crate::model_key::ModelKey;
use crate::{StateRepository, EVENT_PREFIX};
use async_trait::async_trait;
use eventstore::{RecordedEvent, StreamPosition, SubscribeToStreamOptions};
use state::{Event, EventName, State};

pub trait HasTarget {
    fn get_target(&self) -> ModelKey;
}

pub trait CrossData {
    type Question: Event + HasTarget;
    type Answer: Event + HasTarget;

    fn question_names() -> Vec<EventName>;
    fn answer_names() -> Vec<EventName>;
}

pub trait CrossDataProcessor: State {
    fn process(repo: StateRepository, event_name: EventName) {
        let stream_name = format!("$et-{}.{}", EVENT_PREFIX, event_name);

        println!("stream_name : {stream_name}");

        tokio::spawn(async move {
            let options = SubscribeToStreamOptions::default()
                .start_from(StreamPosition::End)
                .resolve_link_tos();

            let mut stream = repo
                .event_db
                .clone()
                .subscribe_to_stream(stream_name, &options)
                .await;

            loop {
                let event_json = stream.next().await.unwrap();
                let repo = repo.clone();

                if let Some(recorded_event) = event_json.event {
                    tokio::spawn(async move {
                        let mut metadata: Metadata =
                            serde_json::from_slice(recorded_event.custom_metadata.as_ref())
                                .unwrap();
                        metadata.set_id(Some(recorded_event.id));

                        let repo = repo.clone();

                        let local_key: ModelKey = recorded_event.stream_id.clone().into();

                        let (cmd, target) = Self::resolve(recorded_event, local_key);

                        repo.add_command::<Self>(&target, cmd, Some(&metadata))
                            .await
                            .unwrap();
                    });
                }
            }
        });
    }

    fn resolve(e: RecordedEvent, local_key: ModelKey) -> (Self::Command, ModelKey);
}

#[async_trait]
pub trait CrossStateQuestion<C>: CrossDataProcessor
where
    C: CrossData,
{
    fn resolve_question(event: C::Question, local_key: ModelKey) -> Self::Command;

    async fn process_question(repo: StateRepository) {
        for event_name in C::question_names() {
            Self::process(repo.clone(), event_name);
        }
    }

    fn resolve_helper(e: RecordedEvent, local_key: ModelKey) -> (Self::Command, ModelKey) {
        let event = e.as_json::<C::Question>().unwrap();
        let target = event.get_target();
        let cmd = Self::resolve_question(event, local_key);
        (cmd, target)
    }
}

#[async_trait]
pub trait CrossStateAnswer<C>: CrossDataProcessor
where
    C: CrossData,
{
    fn resolve_answer(event: C::Answer) -> Self::Command;

    async fn process_query(repo: StateRepository) {
        for event_name in C::answer_names() {
            Self::process(repo.clone(), event_name);
        }
    }

    fn resolve_helper(e: RecordedEvent) -> (Self::Command, ModelKey) {
        let event = e.as_json::<C::Answer>().unwrap();
        let target = event.get_target();
        let cmd = Self::resolve_answer(event);
        (cmd, target)
    }
}
