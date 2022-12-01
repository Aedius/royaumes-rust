#![feature(associated_type_defaults)]

use eventstore::{StreamPosition, SubscribeToStreamOptions};
use state::{Command, Notification, State};
use state_repository::{Metadata, ModelKey, StateRepository};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct DelayedCommand<C>
where
    C: Command,
{
    pub command: C,
    pub delay: Duration,
}

pub trait DelayedCommandFromNotification<N, C>
where
    N: Notification + Send + Sync,
    C: Command + Send + Sync,
{
    fn get_command(notification: N, notification_state_key: ModelKey) -> Option<DelayedCommand<C>>;
}

pub async fn process_wait<T, S>(repo: StateRepository, notifications: Vec<&'static str>)
where
    S: State + Send + 'static,
    T: Notification + Send + Sync,
    S::Command: Send + Sync,
    S::Event: Send + Sync,
    S::Command: DelayedCommandFromNotification<T, S::Command>,
{
    for notification_name in notifications {
        let repo = repo.clone();
        let event_db = repo.event_db().clone();

        let stream_name = format!("$et-ntf.{}.{}", S::name_prefix(), notification_name);
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

                    let local_key: ModelKey = e.stream_id.into();

                    if let Some(cmd) = S::Command::get_command(notification, local_key.clone()) {
                        tokio::spawn(async move {
                            sleep(cmd.delay).await;

                            repo.add_command::<S>(&local_key, cmd.command, Some(metadata))
                                .await
                                .unwrap();
                        });
                    }
                }
            }
        });
    }
}
