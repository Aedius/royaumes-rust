use async_trait::async_trait;
use eventstore::{StreamPosition, SubscribeToStreamOptions};
use state::{EventName, State};
use tokio::time::{sleep, Duration};
use crate::{EVENT_PREFIX, Metadata, ModelKey, StateRepository};

#[async_trait]
pub trait DelayedState: State {
    fn event_to_delayed() -> Vec<EventName>;

    fn resolve_command(event: Self::Event) -> (Self::Command, Duration);

    async fn process_delayed(repo: StateRepository) {
        for event_name in Self::event_to_delayed() {
            let stream_name = format!("$et-{}.{}.{}", EVENT_PREFIX, Self::name_prefix(), event_name);
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

                            let event = e.as_json::<Self::Event>().unwrap();
                            let local_key: ModelKey = e.stream_id.clone().into();

                            let repo = repo.clone();

                            let (cmd, duration) = Self::resolve_command(event);

                            sleep(duration).await;

                            repo.add_command::<Self>(&local_key, cmd, Some(&metadata))
                                .await
                                .unwrap();
                        });
                    }
                }
            });
        }
    }
}