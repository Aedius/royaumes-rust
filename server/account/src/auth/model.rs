use crate::auth::event::AccountEvent;

#[derive(Default, Debug)]
pub struct Account {
    pub name: String,
    pub nb: usize,
}

impl Account {
    pub fn play_event(&mut self, event: AccountEvent) {
        match event {
            AccountEvent::Created(created) => {
                self.name = created.name;
                self.nb = 0;
            }
            AccountEvent::Added(quantity) => {
                self.nb = self.nb.checked_add(quantity.nb).unwrap_or(usize::MAX);
            }
            AccountEvent::Removed(quantity) => {
                self.nb = self.nb.saturating_sub(quantity.nb);
            }
        }
    }
}
