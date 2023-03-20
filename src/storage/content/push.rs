use cid::Cid;

use crate::{errno::Error, storage::Content, utils::codec};

use super::{imports, ReadWriteContent};

impl Content {
    fn push_unsafe(&self, cid: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::contentPushFile(self.id, cid)
        }
    }

    pub fn push(&self) -> Result<Cid, Box<dyn std::error::Error>> {
        let mut reader = codec::cid::Reader::new();

        let err0 = self.push_unsafe(reader.ptr());
        if err0.is_err() {
            Err(format!("Pushing content failed with: {}", err0).into())
        } else {
            Ok(reader.parse()?)
        }
    }
}

impl ReadWriteContent {
    pub fn push(&self) -> Result<Cid, Box<dyn std::error::Error>> {
        self.content.push()
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

        let cid = read_write.push().unwrap_or_else(|err| {
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

    pub fn contentPushFile(id: u32, cid_ptr: *mut u8) -> Error {
        use super::test;

        if id != new_test::ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_cid_string(test::CID, cid_ptr);
            Errno::ErrorNone.error()
        }
    }
}
