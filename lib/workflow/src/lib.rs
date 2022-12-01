#![feature(associated_type_defaults)]

use eventstore::{AppendToStreamOptions, EventData, StreamPosition, SubscribeToStreamOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;
use state::State;
use state_repository::{Metadata, ModelKey, StateRepository};
use std::fmt::Debug;
use uuid::Uuid;

pub trait Transfert: Serialize + DeserializeOwned + Debug + Send + Clone {
    fn transfert_name(&self) -> &str;
    fn transfert_names() -> Vec<&'static str>;

    fn target(&self) -> &ModelKey;
}

pub fn to_transfert_data<T: Transfert>(transfert: &T, previous: Metadata) -> (EventData, Metadata) {
    let id = Uuid::new_v4();
    let mut event_data =
        EventData::json(format!("tft.{}", transfert.transfert_name()), transfert).unwrap();
    event_data = event_data.id(id);

    let metadata = Metadata::new(
        Some(id),
        previous.correlation_id(),
        match previous.id() {
            None => id,
            Some(uuid) => uuid,
        },
        false,
    );

    event_data = event_data.metadata_as_json(&metadata).unwrap();

    (event_data, metadata)
}

pub trait Flow {
    type Input: Transfert;
    type Output: Transfert;
}

pub trait Distant<F>: State
where
    F: Flow,
    <F as Flow>::Input: Sync,
    Self: Send,
    <Self as State>::Command: Sync,
    <Self as State>::Event: Send,
{
    fn get_command(input: F::Input) -> Self::Command;

    fn get_response(output: Self::Notification) -> Option<F::Output>;

    fn process_flow(repo: StateRepository) {
        for notification_name in F::Input::transfert_names() {
            let repo = repo.clone();
            let event_db = repo.event_db().clone();

            let stream_name = format!("$et-tft.{}", notification_name);
            tokio::spawn(async move {
                let options = SubscribeToStreamOptions::default()
                    .start_from(StreamPosition::End)
                    .resolve_link_tos();

                let mut stream = event_db.subscribe_to_stream(stream_name, &options).await;

                loop {
                    let notification_json = stream.next().await.unwrap();

                    if let Some(e) = notification_json.event {
                        let mut metadata: Metadata =
                            serde_json::from_slice(e.custom_metadata.as_ref()).unwrap();
                        metadata.set_id(Some(e.id));

                        let transfert = e.as_json::<F::Input>().unwrap();

                        let repo = repo.clone();

                        let local_key: ModelKey = e.stream_id.into();

                        let cmd = Self::get_command(transfert.clone());

                        let (_, notifications) = repo
                            .add_command::<Self>(transfert.target(), cmd, Some(metadata.clone()))
                            .await
                            .unwrap();

                        for notification in notifications {
                            if let Some(transfert) = Self::get_response(notification) {
                                let (event_data, _) =
                                    to_transfert_data(&transfert, metadata.clone()); // FIXME metadata is wrong

                                repo.try_append_event_data(
                                    &local_key,
                                    &AppendToStreamOptions::default(),
                                    vec![event_data],
                                )
                                .await
                                .unwrap();
                            }
                        }
                    }
                }
            });
        }
    }
}
