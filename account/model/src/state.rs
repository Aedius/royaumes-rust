use std::collections::HashMap;

use chrono::{TimeZone, Utc};
use rocket::form::validate::Contains;
use uuid::Uuid;

use account_api::{AccountCommand, AccountDto};
use anyhow::{anyhow, Result};
use rocket::serde::{Deserialize, Serialize};
use state::State;

use crate::event::{Created, LoggedIn, Quantity, ServerAccount};
use crate::{AccountError, AccountEvent};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountState {
    uuid: Uuid,
    pseudo: String,
    last_login: String,
    nb_account_allowed: usize,
    accounts: HashMap<String, Vec<String>>,
    nb_accounts: usize,
    position: u64,
}

impl AccountState {
    pub fn dto(&self) -> AccountDto {
        AccountDto {
            pseudo: self.pseudo.clone(),
            nb: self.nb_accounts,
        }
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub fn pseudo(&self) -> &str {
        &self.pseudo
    }
    pub fn last_login(&self) -> &str {
        &self.last_login
    }
    pub fn nb_account_allowed(&self) -> usize {
        self.nb_account_allowed
    }
    pub fn accounts(&self) -> &HashMap<String, Vec<String>> {
        &self.accounts
    }
    pub fn nb_accounts(&self) -> usize {
        self.nb_accounts
    }
    pub fn position(&self) -> u64 {
        self.position
    }
}

impl State for AccountState {
    type Event = AccountEvent;
    type Command = AccountCommand;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            AccountEvent::Created(created) => {
                self.uuid = created.uuid;
                self.pseudo = created.pseudo.clone();
                self.nb_account_allowed = 0;
            }
            AccountEvent::AccountAdded(quantity) => {
                self.nb_account_allowed = self
                    .nb_account_allowed
                    .checked_add(quantity.nb)
                    .unwrap_or(usize::MAX);
            }
            AccountEvent::AccountRemoved(quantity) => {
                self.nb_account_allowed = self.nb_account_allowed.saturating_sub(quantity.nb);
            }
            AccountEvent::Logged(log) => {
                let date = Utc.timestamp(log.time.try_into().expect("timestamp is too big"), 0);

                self.last_login = date.to_rfc2822()
            }
            AccountEvent::Joined(sa) => {
                match self.accounts.get_mut(&*sa.server_id) {
                    None => {
                        self.accounts
                            .insert(sa.server_id.clone(), vec![sa.account_id.clone()]);
                    }
                    Some(list) => {
                        list.push(sa.account_id.clone());
                    }
                }
                self.nb_accounts += 1;
            }
            AccountEvent::Leaved(sa) => {
                if let Some(accounts) = self.accounts.get_mut(&*sa.server_id) {
                    accounts.retain(|x| x != &sa.account_id);
                }
                self.nb_accounts -= 1;
            }
        }
    }

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>> {
        match command {
            AccountCommand::CreateAccount(create) => {
                if !self.pseudo.is_empty() {
                    Err(anyhow!(AccountError::Other(
                        "account already has a pseudo".to_string(),
                    )))
                } else {
                    Ok(vec![AccountEvent::Created(Created {
                        uuid: Uuid::new_v4(),
                        pseudo: create.pseudo.clone(),
                    })])
                }
            }
            AccountCommand::Login(login) => {
                Ok(vec![AccountEvent::Logged(LoggedIn { time: login.time })])
            }
            AccountCommand::AddQuantity(nb) => {
                if self.nb_account_allowed.checked_add(*nb).is_none() {
                    Err(anyhow!(AccountError::WrongQuantity(format!(
                        "cannot add {} to {}",
                        nb, self.nb_account_allowed
                    ))))
                } else {
                    Ok(vec![AccountEvent::AccountAdded(Quantity { nb: *nb })])
                }
            }
            AccountCommand::RemoveQuantity(nb) => {
                if nb > &self.nb_account_allowed {
                    Err(anyhow!(AccountError::WrongQuantity(format!(
                        "cannot remove {} from {}",
                        nb, self.nb_account_allowed
                    ))))
                } else {
                    Ok(vec![AccountEvent::AccountRemoved(Quantity { nb: *nb })])
                }
            }
            AccountCommand::Join(join) => {
                if self.nb_accounts >= self.nb_account_allowed {
                    return Err(anyhow!(AccountError::Other(
                        "already maximum accounts".to_string()
                    )));
                }

                match self.accounts.get(&*join.server_id) {
                    None => Ok(vec![AccountEvent::Joined(ServerAccount {
                        server_id: join.server_id.clone(),
                        account_id: join.account_id.clone(),
                    })]),
                    Some(list) => {
                        if list.contains(join.account_id.clone()) {
                            Err(anyhow!(AccountError::Other("Already joined".to_string())))
                        } else {
                            Ok(vec![AccountEvent::Joined(ServerAccount {
                                server_id: join.server_id.clone(),
                                account_id: join.account_id.clone(),
                            })])
                        }
                    }
                }
            }
            AccountCommand::Leave(leave) => {
                if self.nb_accounts == 0 {
                    return Err(anyhow!(AccountError::Other("No account yet".to_string())));
                }
                match self.accounts.get(&*leave.server_id) {
                    Some(list) => {
                        if list.contains(leave.account_id.clone()) {
                            Ok(vec![AccountEvent::Leaved(ServerAccount {
                                server_id: leave.server_id.clone(),
                                account_id: leave.account_id.clone(),
                            })])
                        } else {
                            Err(anyhow!(AccountError::Other(
                                "account not found".to_string()
                            )))
                        }
                    }
                    None => Err(anyhow!(AccountError::Other(
                        "no account on the server".to_string()
                    ))),
                }
            }
        }
    }

    fn get_position(&self) -> u64 {
        self.position
    }

    fn set_position(&mut self, pos: u64) {
        self.position = pos;
    }

    fn state_cache_interval() -> Option<u64> {
        Some(20)
    }
}
