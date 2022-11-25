#![feature(associated_type_defaults)]

use eventstore::{StreamPosition, SubscribeToStreamOptions};
use state::{Command, Event, State};
use state_repository::{ModelKey, StateRepository};
use tokio::time::{sleep, Duration};

pub trait WaitingState<T>: State + Send
where T : 'static + Command + Send + Sync {
    fn get_next(event: &Self::Event) -> Option<(T, Duration)>;
}

pub async fn process_wait<U, T: WaitingState<U> + State<Command = U>>(repo: StateRepository, event: T::Event)
where U : 'static + Command + Send + Sync , <T as State>::Event: Send
{
    let event_db = repo.event_db().clone();
    let stream_name = format!("$et-{}.{}", T::Event::name_prefix(), event.event_name());

    tokio::spawn(async move {
        let options = SubscribeToStreamOptions::default()
            .start_from(StreamPosition::End)
            .resolve_link_tos();

        let mut stream = event_db.subscribe_to_stream(stream_name, &options).await;

        loop {
            let event_json = stream.next().await.unwrap();

            if let Some(e) = event_json.event {
                println!("{:?}", e.stream_id);

                let event: T::Event = e.as_json::<T::Event>().unwrap();

                println!("{:?}", event);

                let repo = repo.clone();

                if let Some((c, d)) = T::get_next(&event) {
                    tokio::spawn(async move {
                        sleep(d).await;
                        let key: ModelKey = e.stream_id.into();
                        println!("{key:?}");
                        repo.add_command::<T>(&key, c).await.unwrap();
                    });
                }
            }
        }
    });
}
