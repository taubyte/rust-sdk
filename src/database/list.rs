use super::{imports, Database};
use crate::{errno::Error, utils::codec};

impl Database {
    fn list_size_unsafe(&self, key: &str, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::databaseListSize(self.id, key.as_ptr(), key.len(), size)
        }
    }

    fn list_unsafe(&self, key: &str, buf_ptr: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::databaseList(self.id, key.as_ptr(), key.len(), buf_ptr)
        }
    }

    pub fn list(&self, key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.list_size_unsafe(key, &mut size);
        if err0.is_err() {
            return Err(format!(
                "Getting database size for key: `{}` failed with: {}",
                key, err0
            )
            .into());
        }
        if size == 0 {
            return Ok(Vec::new());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.list_unsafe(key, buf.as_mut_ptr());
        if err0.is_err() {
            Err(format!("Getting database for key: `{}` failed with: {}", key, err0).into())
        } else {
            Ok(codec::string_slice::to(buf))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::database::{new::test, Database};

    pub static KEY: &str = "/test/v1";
    pub static SUB_KEYS: [&str; 2] = ["a", "b"];

    #[test]
    fn test_list() {
        let database = Database::new(test::DATABASE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let value = database.list(KEY).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(value, SUB_KEYS)
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

    pub fn databaseList(id: u32, key_ptr: *const u8, key_size: usize, data: *mut u8) -> Error {
        let key = utils::read_string(key_ptr, key_size);

        if id != new_test::DATABASE_ID {
            Errno::ErrorCap.error()
        } else if key != test::KEY {
            Errno::ErrorCap.error()
        } else {
            utils::write_string_slice(data, &test::SUB_KEYS);
            Errno::ErrorNone.error()
        }
    }

    pub fn databaseListSize(
        id: u32,
        key_ptr: *const u8,
        key_size: usize,
        size: *mut usize,
    ) -> Error {
        let key = utils::read_string(key_ptr, key_size);

        if id != new_test::DATABASE_ID {
            Errno::ErrorCap.error()
        } else if key != test::KEY {
            Errno::ErrorCap.error()
        } else {
            utils::write_string_slice_size(size, &test::SUB_KEYS);
            Errno::ErrorNone.error()
        }
    }
}
