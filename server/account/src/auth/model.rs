use crate::auth::event::AccountEvent;

#[derive(Default, Debug)]
pub struct Account {
    name: String,
    nb: usize,
}

impl Account {
    pub fn play_event(&mut self, event: AccountEvent) -> Result<(), String> {
        match event {
            AccountEvent::Created(created) => {
                self.name = created.name;
                self.nb = 0;
            }
            AccountEvent::Added(quantity) => {
                self.nb += quantity.nb;
            }
            AccountEvent::Removed(quantity) => {
                self.nb -= quantity.nb;
            }
        }

        Ok(())
    }
}
