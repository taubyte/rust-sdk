mod cid;
mod close;
mod new;
mod open;
mod push;
mod read;
mod seek;
mod write;

use super::Content;

pub struct ReadWriteContent {
    content: Content,
}

pub struct ReadOnlyContent {
    content: Content,
}

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

#[cfg(test)]
mod imports {
    pub use super::cid::mock::*;
    pub use super::close::mock::*;
    pub use super::new::mock::*;
    pub use super::open::mock::*;
    pub use super::push::mock::*;
    pub use super::read::mock::*;
    pub use super::seek::mock::*;
    pub use super::write::mock::*;
}
