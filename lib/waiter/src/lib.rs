#![feature(associated_type_defaults)]

use eventstore::{StreamPosition, SubscribeToStreamOptions};
use state::{Command, Event, State};
use state_repository::{Metadata, ModelKey, StateRepository};
use tokio::time::{sleep, Duration};

pub trait CommandFromEvent<E, C>
where
    E: Event + Send + Sync,
    C: Command + Send + Sync,
{
    fn get_command(event: E) -> Option<(C, Option<ModelKey>, Option<Duration>)>;
}

pub async fn process_wait<T, V>(repo: StateRepository, event_name: &'static str)
where
    T: State,
    V: State + Send + 'static,
    T::Event: Send + Sync,
    V::Command: Send + Sync,
    V::Event: Send + Sync,
    V::Command: CommandFromEvent<T::Event, V::Command>,
{
    let event_db = repo.event_db().clone();
    let stream_name = format!("$et-{}.{}", T::Event::name_prefix(), event_name);

    tokio::spawn(async move {
        let options = SubscribeToStreamOptions::default()
            .start_from(StreamPosition::End)
            .resolve_link_tos();

        let mut stream = event_db.subscribe_to_stream(stream_name, &options).await;

        loop {
            let event_json = stream.next().await.unwrap();

            if let Some(e) = event_json.event {
                let mut metadata: Metadata =
                    serde_json::from_slice(e.custom_metadata.as_ref()).unwrap();
                metadata.set_id(Some(e.id));

                let event: T::Event = e.as_json::<T::Event>().unwrap();

                let repo = repo.clone();

                if let Some((command, model_key, duration)) = V::Command::get_command(event) {
                    tokio::spawn(async move {
                        if let Some(d) = duration {
                            sleep(d).await;
                        }

                        let key: ModelKey = match model_key {
                            None => e.stream_id.into(),
                            Some(k) => k,
                        };

                        repo.add_command::<V>(&key, command, Some(metadata))
                            .await
                            .unwrap();
                    });
                }
            }
        }
    });
}
