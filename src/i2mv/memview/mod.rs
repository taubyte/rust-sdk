use crate::{
    errno::Error,
    utils::booleans::convert::{from_bool, to_bool},
};

mod closer;
mod read_seek_closer;

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

pub struct ReadSeekCloser {
    pub id: u32,
    pub size: usize,
    pub offset: i64,
    closable: bool,
}

pub struct Closer {
    pub id: u32,
}

#[cfg(test)]
mod imports {
    pub use super::closer::mock::*;
    pub use super::read_seek_closer::mock::*;
}
