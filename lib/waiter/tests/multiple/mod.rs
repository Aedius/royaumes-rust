pub mod gold;
pub mod build;
pub mod worker;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cost {
    pub gold: u32,
    pub worker: u32,
}
