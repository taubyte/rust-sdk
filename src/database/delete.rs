use super::{imports, Database};
use crate::errno::Error;

impl Database {
    fn delete_unsafe(&self, key: &str) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::databaseDelete(self.id, key.as_ptr(), key.len())
        }
    }

    pub fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let err = self.delete_unsafe(key);
        if err.is_err() {
            Err(format!("Deleting database key: `{}` failed with: {}", key, err).into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::database::{new::test, Database};
    pub static KEY: &str = "/test/v1";

    #[test]
    fn test_delete() {
        let database = Database::new(test::DATABASE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        database.delete(KEY).unwrap_or_else(|err| {
            panic!("{}", err);
        })
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;
    use crate::{
        database::new::test as new_test,
        errno::{Errno, Error},
        utils::test as utils,
    };

    pub fn databaseDelete(id: u32, key_ptr: *const u8, key_size: usize) -> Error {
        let key = utils::read_string(key_ptr, key_size);

        if id != new_test::DATABASE_ID {
            Errno::ErrorCap.error()
        } else if key != test::KEY {
            Errno::ErrorCap.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}
