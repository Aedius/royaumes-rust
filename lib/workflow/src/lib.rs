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
    <F as Flow>::Output: Sync,
    Self: Send,
    <Self as State>::Command: Sync,
    <Self as State>::Event: Send,
    <Self as State>::Notification: Send,
{
    fn get_command(input: F::Input) -> Self::Command;

    fn get_response(output: Self::Notification) -> F::Output;

    fn get_notification_response() -> Vec<&'static str>;

    fn listen(repo: StateRepository) {
        Self::process_input(repo.clone());
        Self::process_output(repo);
    }

    fn process_input(repo: StateRepository) {
        for transfert_name in F::Input::transfert_names() {
            let repo = repo.clone();
            let event_db = repo.event_db().clone();

            let stream_name = format!("$et-tft.{}", transfert_name);

            tokio::spawn(async move {
                let options = SubscribeToStreamOptions::default()
                    .start_from(StreamPosition::End)
                    .resolve_link_tos();

                println!("input : {stream_name}");
                let mut stream = event_db.subscribe_to_stream(stream_name, &options).await;

                loop {
                    let transfert_json = stream.next().await.unwrap();

                    println!("transfert_json : {transfert_json:?}");

                    if let Some(e) = transfert_json.event {
                        let mut metadata: Metadata =
                            serde_json::from_slice(e.custom_metadata.as_ref()).unwrap();
                        metadata.set_id(Some(e.id));

                        let transfert = e.as_json::<F::Input>().unwrap();

                        println!("transfert : {transfert:?}");

                        let repo = repo.clone();

                        let cmd = Self::get_command(transfert.clone());

                        println!("cmd : {cmd:?}");

                        let res = repo
                            .add_command::<Self>(transfert.target(), cmd, Some(metadata.clone()))
                            .await;
                        println!("res : {res:#?}");
                    }
                }
            });
        }
    }

    fn process_output(repo: StateRepository) {
        for notification_name in Self::get_notification_response() {
            let repo = repo.clone();
            let event_db = repo.event_db().clone();
            let stream_name = format!("$et-ntf.{}.{}", Self::name_prefix(), notification_name);

            tokio::spawn(async move {
                let options = SubscribeToStreamOptions::default()
                    .start_from(StreamPosition::End)
                    .resolve_link_tos();

                println!("output : {stream_name}");
                let mut stream = event_db.subscribe_to_stream(stream_name, &options).await;

                loop {
                    let notification_json = stream.next().await;

                    println!("notification_json : {notification_json:?}");

                    let notification_json = notification_json.unwrap();

                    if let Some(e) = notification_json.event {
                        let mut metadata: Metadata =
                            serde_json::from_slice(e.custom_metadata.as_ref()).unwrap();
                        metadata.set_id(Some(e.id));

                        let notification = e.as_json::<Self::Notification>().unwrap();

                        let transfert = Self::get_response(notification);

                        let (event_data, _) = to_transfert_data(&transfert, metadata);

                        repo.try_append_event_data(
                            transfert.target(),
                            &AppendToStreamOptions::default(), // dont take care of the offset
                            vec![event_data],
                        )
                        .await
                        .unwrap();
                    }
                }
            });
        }
    }
}
