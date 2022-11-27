use anyhow::Result;

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait Command: Serialize + DeserializeOwned + Debug + Send + Clone {
    fn command_name(&self) -> &str;
}

pub trait Event: Serialize + DeserializeOwned + Debug + Clone {
    fn event_name(&self) -> &str;
}

pub trait Notification: Serialize + DeserializeOwned + Debug + Send + Clone {
    fn notification_name(&self) -> &str;
}

pub struct Events<T, U>
where
    T: Event,
    U: Notification,
{
    event: Vec<T>,
    notification: Vec<U>,
}

impl Notification for () {
    fn notification_name(&self) -> &str {
        "noop"
    }
}

impl<T: Event, U: Notification> Events<T, U> {
    pub fn new(event: Vec<T>, notification: Vec<U>) -> Self {
        Self {
            event,
            notification,
        }
    }

    pub fn event(&self) -> &Vec<T> {
        &self.event
    }
    pub fn notification(&self) -> &Vec<U> {
        &self.notification
    }
}

pub trait State: Default + Serialize + DeserializeOwned + Debug {
    type Event: Event;
    type Notification: Notification;
    type Command: Command;

    fn name_prefix() -> &'static str;

    fn play_event(&mut self, event: &Self::Event);

    fn try_command(
        &self,
        command: &Self::Command,
    ) -> Result<Events<Self::Event, Self::Notification>>;

    fn get_position(&self) -> u64;

    fn set_position(&mut self, pos: u64);

    fn state_cache_interval() -> Option<u64>;
}
