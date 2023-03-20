use crate::{
    errno::{Errno, Error},
    utils::booleans::convert::{from_bool, to_bool},
};

mod read_closer;
mod write_closer;

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

#[cfg(test)]
mod imports {
    pub use super::read_closer::mock::*;
    pub use super::write_closer::mock::*;
}

pub struct WriteCloser {
    pub id: u32,
}

pub struct ReadCloser {
    pub id: u32,
    pub closable: bool,
}
