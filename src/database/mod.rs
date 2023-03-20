mod close;
mod delete;
mod get;
mod list;
mod new;
mod put;

#[derive(Copy, Clone)]
pub struct Database {
    pub id: u32,
}

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

#[cfg(test)]
mod imports {
    pub use super::close::mock::*;
    pub use super::delete::mock::*;
    pub use super::get::mock::*;
    pub use super::list::mock::*;
    pub use super::new::mock::*;
    pub use super::put::mock::*;
}
