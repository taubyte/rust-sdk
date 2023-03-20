use cid::Cid;

use super::{imports, Storage};
use crate::{errno::Error, utils::codec};

impl Storage {
    fn cid_unsafe(&self, name_ptr: *const u8, name_size: usize, cid: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageCid(self.id, name_ptr, name_size, cid)
        }
    }

    pub fn cid(&self, name: &str) -> Result<Cid, Box<dyn std::error::Error>> {
        let mut reader = codec::cid::Reader::new();

        let err = self.cid_unsafe(name.as_ptr(), name.len(), reader.ptr());
        if err.is_err() {
            Err(format!("Getting storage cid failed with: {}", err).into())
        } else {
            Ok(reader.parse()?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static FILE_NAME: &str = "someFile";
    pub static MOCK_CID: &str = "QmQUXYRNqU2U351aE8mrPFAyqXtKupF9bDspXKLsdkTLGn";

    #[test]
    fn test_cid() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let cid = storage.cid(FILE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(cid.to_string(), MOCK_CID);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        storage::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageCid(id: u32, name_ptr: *const u8, name_size: usize, cid_ptr: *mut u8) -> Error {
        use super::test;

        let name = utils::read_string(name_ptr, name_size);

        if id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if name != test::FILE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_cid_string(test::MOCK_CID, cid_ptr);
            Errno::ErrorNone.error()
        }
    }
}
