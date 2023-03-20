mod capacity;
mod cid;
mod content;
mod file;
mod files;
mod get;
mod new;

#[derive(Copy, Clone)]
pub struct Storage {
    pub id: u32,
}

pub struct Content {
    pub id: u32,
    consumed: bool,
}

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

#[cfg(test)]
mod imports {
    pub use super::capacity::mock::*;
    pub use super::cid::mock::*;
    pub use super::files::mock::*;
    pub use super::get::mock::*;
    pub use super::new::mock::*;
}
