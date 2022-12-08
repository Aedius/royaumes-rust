use std::time::SystemTime;

use uuid::Uuid;

use account_shared::{AccountCommand, AccountDto};
use anyhow::{anyhow, Result};
use rocket::serde::{Deserialize, Serialize};
use state::State;

use crate::event::{Created, LoggedIn};
use crate::{AccountError, AccountEvent};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct AccountState {
    uuid: Uuid,
    pseudo: String,
    register_at: u64,
    last_login: u64,
    reputation: usize,
    position: Option<u64>,
}

impl AccountState {
    pub fn dto(&self) -> AccountDto {
        AccountDto {
            pseudo: self.pseudo.clone(),
            reputation: self.reputation,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub fn pseudo(&self) -> &str {
        &self.pseudo
    }
    pub fn register_at(&self) -> u64 {
        self.register_at
    }
    pub fn last_login(&self) -> u64 {
        self.last_login
    }
    pub fn reputation(&self) -> usize {
        self.reputation
    }
}

impl State for AccountState {
    type Event = AccountEvent;
    type Command = AccountCommand;

    fn name_prefix() -> &'static str {
        "account"
    }

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            AccountEvent::Created(created) => {
                self.uuid = created.uuid;
                self.pseudo = created.pseudo.clone();
                self.register_at = created.time;
            }
            AccountEvent::ReputationAdded(quantity) => {
                self.reputation = self.reputation.checked_add(*quantity).unwrap_or(usize::MAX);
            }
            AccountEvent::ReputationRemoved(quantity) => {
                self.reputation = self.reputation.saturating_sub(*quantity);
            }
            AccountEvent::Logged(log) => self.last_login = log.time,
        }
    }

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>> {
        match command {
            AccountCommand::CreateAccount(create) => {
                if !self.pseudo.is_empty() {
                    Err(anyhow!(AccountError::Other(
                        "account already has a pseudo".to_string(),
                    )))
                } else {
                    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

                    Ok(vec![AccountEvent::Created(Created {
                        uuid: Uuid::new_v4(),
                        pseudo: create.pseudo,
                        time: now.as_secs(),
                    })])
                }
            }
            AccountCommand::Login(_login) => {
                let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

                Ok(vec![AccountEvent::Logged(LoggedIn {
                    time: now.as_secs(),
                })])
            }
            AccountCommand::AddReputation(nb) => {
                if self.reputation.checked_add(nb).is_none() {
                    Err(anyhow!(AccountError::WrongQuantity(format!(
                        "cannot add {} to {}",
                        nb, self.reputation
                    ))))
                } else {
                    Ok(vec![AccountEvent::ReputationAdded(nb)])
                }
            }
            AccountCommand::RemoveReputation(nb) => {
                if nb > self.reputation {
                    Err(anyhow!(AccountError::WrongQuantity(format!(
                        "cannot remove {} from {}",
                        nb, self.reputation
                    ))))
                } else {
                    Ok(vec![AccountEvent::ReputationRemoved(nb)])
                }
            }
        }
    }

    fn state_cache_interval() -> Option<u64> {
        Some(20)
    }
}
