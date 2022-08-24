use crate::auth::event::AccountEvent;
use crate::auth::Account;
use account_api::AccountDto;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct AccountModel {
    pub uuid: Uuid,
    pub pseudo: String,
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
            },
            Account::Command(_) => {}
            Account::Error(_) => {}
        }
    }

    pub fn dto(&self) -> AccountDto {
        AccountDto {
            uuid: self.uuid.to_string(),
            pseudo: self.pseudo.clone(),
            nb: self.nb,
        }
    }
}
