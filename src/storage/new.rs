use super::{imports, Storage};
use crate::errno::Error;

fn new_storage(name: &str, id: *mut u32) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::storageNew(name.as_ptr(), name.len(), id)
    }
}

impl Storage {
    pub fn new(name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err = new_storage(name, &mut id);
        if err.is_err() {
            Err(format!("Creating storage failed with: {}", err).into())
        } else {
            Ok(Storage { id })
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::Storage;

    pub static STORAGE_ID: u32 = 1;
    pub static STORAGE_NAME: &str = "testStorage";

    #[test]
    fn test_new() {
        let storage = Storage::new(STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(storage.id, STORAGE_ID)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        utils::test as utils,
    };

    pub fn storageNew(name_ptr: *const u8, name_size: usize, id: *mut u32) -> Error {
        use super::test;

        let name = utils::read_string(name_ptr, name_size);
        if name != test::STORAGE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_u32(id, test::STORAGE_ID);
            Errno::ErrorNone.error()
        }
    }
}
