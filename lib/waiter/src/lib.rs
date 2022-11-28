#![feature(associated_type_defaults)]

use eventstore::{StreamPosition, SubscribeToStreamOptions};
use state::{Command, Notification, State};
use state_repository::{Metadata, ModelKey, StateRepository};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct DeportedCommand<C>
where
    C: Command,
{
    pub command: C,
    pub target_state_key: ModelKey,
    pub duration: Option<Duration>,
}

pub trait CommandFromNotification<N, C>
where
    N: Notification + Send + Sync,
    C: Command + Send + Sync,
{
    fn get_command(notification: N, notification_state_key: ModelKey)
        -> Option<DeportedCommand<C>>;
}

pub async fn process_wait<T, V>(repo: StateRepository, notifications: Vec<&'static str>)
where
    V: State + Send + 'static,
    T: Notification + Send + Sync,
    V::Command: Send + Sync,
    V::Event: Send + Sync,
    V::Command: CommandFromNotification<T, V::Command>,
{
    for notification_name in notifications {
        let repo = repo.clone();
        let event_db = repo.event_db().clone();

        let stream_name = format!("$et-ntf.{}", notification_name);
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

                    let notification: T = e.as_json::<T>().unwrap();

                    let repo = repo.clone();

                    if let Some(cmd) = V::Command::get_command(notification, e.stream_id.into()) {
                        tokio::spawn(async move {
                            if let Some(d) = cmd.duration {
                                sleep(d).await;
                            }

                            repo.add_command::<V>(
                                &cmd.target_state_key,
                                cmd.command,
                                Some(metadata),
                            )
                            .await
                            .unwrap();
                        });
                    }
                }
            }
        });
    }
}
