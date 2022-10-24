#![feature(associated_type_defaults)]

use eventstore::EventData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Metadata {
    #[serde(rename = "$correlationId")]
    correlation_id: Uuid,
}

pub trait Event: Serialize {
    fn event_name(&self) -> &str;

    fn to_event_data(&self, previous: Option<Uuid>) -> (EventData, Uuid) {
        let id = Uuid::new_v4();
        let mut event_data = EventData::json(self.event_name(), self).unwrap();
        event_data = event_data.id(id);

        if let Some(previous_uuid) = previous {
            event_data = event_data
                .metadata_as_json(Metadata {
                    correlation_id: previous_uuid,
                })
                .unwrap();
        }

        (event_data, id)
    }
}

pub enum Operator<A, B> {
    Public(A),
    Private(B),
}

pub trait EventModel {
    type PublicEvent: Event;
    type PrivateEvent: Event;
    type PublicCommand;
    type PrivateCommand;
    type Error;

    type Event = Operator<Self::PublicEvent, Self::PrivateEvent>;

    fn play_event(&mut self, event: &Operator<Self::PublicEvent, Self::PrivateEvent>) {
        match event {
            Operator::Public(p) => self.play_public_event(p),
            Operator::Private(p) => self.play_private_event(p),
        }
    }

    fn play_public_event(&mut self, event: &Self::PublicEvent);
    fn play_private_event(&mut self, event: &Self::PrivateEvent);

    fn try_command(
        &self,
        command: &Operator<Self::PublicCommand, Self::PrivateCommand>,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            Operator::Public(p) => self.try_public_command(p),
            Operator::Private(p) => self.try_private_command(p),
        }
    }

    fn try_public_command(
        &self,
        command: &Self::PublicCommand,
    ) -> Result<Vec<Self::Event>, Self::Error>;
    fn try_private_command(
        &self,
        command: &Self::PrivateCommand,
    ) -> Result<Vec<Self::Event>, Self::Error>;
}
