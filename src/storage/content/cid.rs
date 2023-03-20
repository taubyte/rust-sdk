use cid::Cid;

use crate::{errno::Error, utils::codec};

use super::{imports, ReadWriteContent};

impl ReadWriteContent {
    fn cid_unsafe(&self, cid: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::contentFileCid(self.content.id, cid)
        }
    }

    pub fn cid(&self) -> Result<Cid, Box<dyn std::error::Error>> {
        let mut reader = codec::cid::Reader::new();

        let err = self.cid_unsafe(reader.ptr());
        if err.is_err() {
            Err(format!("Getting storage cid failed with: {}", err).into())
        } else {
            Ok(reader.parse()?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::Content;

    pub static CID: &str = "QmeqqyqBSwyJTSjEyiybuB8NUhJrecQSuNDnjEEE68Dy8z";

    #[test]
    fn cid_read_write() {
        let read_write = Content::new().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let cid = read_write.cid().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(cid.to_string(), CID);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        storage::content::new::test as new_test,
        utils::test as utils,
    };

    pub fn contentFileCid(id: u32, cid_ptr: *mut u8) -> Error {
        use super::test;

        if id != new_test::ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_cid_string(test::CID, cid_ptr);
            Errno::ErrorNone.error()
        }
    }
}
