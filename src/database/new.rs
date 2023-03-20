use super::{imports, Database};
use crate::errno::Error;

fn new_database(name: &str, id: *mut u32) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::newDatabase(name.as_ptr(), name.len(), id)
    }
}

impl Database {
    pub fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err = new_database(name, &mut id);
        if err.is_err() {
            Err(format!("Creating database failed with: {}", err).into())
        } else {
            Ok(Database { id })
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::database::Database;

    pub static DATABASE_ID: u32 = 1;
    pub static DATABASE_NAME: &str = "testDatabase";

    #[test]
    fn test_new() {
        let database = Database::new(DATABASE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(database.id, DATABASE_ID)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;
    use crate::{
        errno::{Errno, Error},
        utils::test as utils,
    };

    pub fn newDatabase(name_ptr: *const u8, name_size: usize, id: *mut u32) -> Error {
        let name = utils::read_string(name_ptr, name_size);
        if name != test::DATABASE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_u32(id, test::DATABASE_ID);
            Errno::ErrorNone.error()
        }
    }
}
