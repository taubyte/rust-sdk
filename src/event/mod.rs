mod events;

#[derive(Copy, Clone)]
pub struct Event {
    pub event: u32,
}

pub use events::EventType;

use std::error::Error;

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

#[cfg(test)]
mod imports {
    pub use super::events::mock::*;
}
