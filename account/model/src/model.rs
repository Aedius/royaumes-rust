use account_api::AccountDto;
use chrono::{TimeZone, Utc};
use uuid::Uuid;
use uuid::Uuid;
use chrono::{ TimeZone, Utc};
use crate::{Account, AccountEvent};

#[derive(Default, Debug)]
pub struct AccountModel {
    pub uuid: Uuid,
    pub pseudo: String,
    pub last_login: String,
    pub nb: usize,
}

impl AccountModel {
    pub fn play_event(&mut self, thing: Account) {
        match thing {
            Account::Event(event) => match event {
                AccountEvent::Created(created) => {
                    self.uuid = created.uuid;
                    self.pseudo = created.pseudo;
                    self.nb = 0;
                }
                AccountEvent::Added(quantity) => {
                    self.nb = self.nb.checked_add(quantity.nb).unwrap_or(usize::MAX);
                }
                AccountEvent::Removed(quantity) => {
                    self.nb = self.nb.saturating_sub(quantity.nb);
                }
                AccountEvent::Logged(log) => {

                    let date = Utc.timestamp(log.time.try_into().expect("timestamp is too big"), 0);

                    self.last_login =date.to_rfc2822()
                }
            },
            Account::Command(_) => {}
            Account::Error(_) => {}
        }
    }

    pub fn dto(&self) -> AccountDto {
        AccountDto {
            pseudo: self.pseudo.clone(),
            nb: self.nb,
        }
    }
}
