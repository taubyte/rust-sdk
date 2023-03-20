use super::{imports, Database};
use crate::errno::Error;

impl Database {
    fn put_unsafe(&self, key: &str, buf_ptr: *mut u8, buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::databasePut(self.id, key.as_ptr(), key.len(), buf_ptr, buf_size)
        }
    }

    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let err = self.put_unsafe(key, value.as_ptr() as *mut u8, value.len());
        if err.is_err() {
            Err(format!("Putting key: `{}` failed with: {}", key, err).into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::database::{new::test, Database};
    pub static KEY: &str = "/test/v1";
    pub static VALUE: &str = "a value";

    #[test]
    fn test_put() {
        let database = Database::new(test::DATABASE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        database.put(KEY, VALUE.as_bytes()).unwrap_or_else(|err| {
            panic!("{}", err);
        });
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

    pub fn databasePut(
        id: u32,
        key_ptr: *const u8,
        key_size: usize,
        data: *mut u8,
        data_size: usize,
    ) -> Error {
        let key = utils::read_string(key_ptr, key_size);

        if id != new_test::DATABASE_ID {
            Errno::ErrorCap.error()
        } else if key != test::KEY {
            Errno::ErrorCap.error()
        } else {
            assert_eq!(utils::read_bytes(data, data_size), test::VALUE.as_bytes());
            Errno::ErrorNone.error()
        }
    }
}
