use async_trait::async_trait;
use eventstore::{  StreamPosition, SubscribeToStreamOptions};
use state::{ Event, EventName, State, StateName};
use crate::{EVENT_PREFIX, StateRepository};
use crate::metadata::Metadata;
use crate::model_key::ModelKey;

pub trait CrossData
{
    type Question: Event;
    type Answer: Event;

    fn questioner() -> StateName;
    fn answerer() -> StateName;

    fn question_names() -> Vec<EventName>;
    fn answer_names() -> Vec<EventName>;
}


#[async_trait]
pub trait CrossStateQuestion<C>: State
where C: CrossData{

    fn resolve_question(event: C::Question, local_key: ModelKey) -> (Self::Command, ModelKey);

    async fn process_question(repo: StateRepository){

        for event_name in C::question_names(){
            let stream_name = format!("$et-{}.{}.{}", EVENT_PREFIX, C::questioner(), event_name);
            let repo = repo.clone();
            tokio::spawn(async move {
                let options = SubscribeToStreamOptions::default()
                    .start_from(StreamPosition::End)
                    .resolve_link_tos();

                let mut stream = repo.event_db.clone().subscribe_to_stream(stream_name, &options).await;

                loop {
                    let event_json = stream.next().await.unwrap();
                    let repo = repo.clone();

                    if let Some(e) = event_json.event {
                        tokio::spawn(async move {
                            let mut metadata: Metadata =
                                serde_json::from_slice(e.custom_metadata.as_ref()).unwrap();
                            metadata.set_id(Some(e.id));

                            let event = e.as_json::<C::Question>().unwrap();
                            let local_key: ModelKey = e.stream_id.clone().into();

                            let repo = repo.clone();

                            let ( cmd, target ) = Self::resolve_question(event, local_key);

                            repo.add_command::<Self>(&target, cmd, Some(&metadata))
                                .await
                                .unwrap();
                        });
                    }
                }
            });
        }
    }
}

#[async_trait]
pub trait CrossStateAnswer<C>: State
where C:CrossData{
    fn resolve_answer(event: C::Answer) -> (Self::Command, ModelKey);

    async fn process_query(repo: StateRepository){

        for event_name in C::answer_names() {
            let stream_name = format!("$et-{}.{}.{}", EVENT_PREFIX, C::answerer(), event_name);
            let repo = repo.clone();
            tokio::spawn(async move {
                let options = SubscribeToStreamOptions::default()
                    .start_from(StreamPosition::End)
                    .resolve_link_tos();

                let mut stream = repo.event_db.clone().subscribe_to_stream(stream_name, &options).await;

                loop {
                    let event_json = stream.next().await.unwrap();
                    let repo = repo.clone();

                    if let Some(e) = event_json.event {
                        tokio::spawn(async move {
                            let mut metadata: Metadata =
                                serde_json::from_slice(e.custom_metadata.as_ref()).unwrap();
                            metadata.set_id(Some(e.id));

                            let event = e.as_json::<C::Answer>().unwrap();

                            let repo = repo.clone();

                            let (cmd, target) = Self::resolve_answer(event);

                            repo.add_command::<Self>(&target, cmd, Some(&metadata))
                                .await
                                .unwrap();
                        });
                    }

                }
            });
        }
    }
}
