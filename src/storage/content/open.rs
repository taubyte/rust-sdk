use crate::errno::Error;

use super::{imports, Content, ReadOnlyContent};
use cid::Cid;

impl Content {
    fn open_unsafe(id: *mut u32, cid: Cid) -> Error {
        let cid_bytes = cid.to_bytes();

        #[allow(unused_unsafe)]
        unsafe {
            imports::storageOpenCid(id, cid_bytes.as_ptr())
        }
    }

    pub fn open(cid: Cid) -> Result<ReadOnlyContent, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err0 = Content::open_unsafe(&mut id, cid);
        if err0.is_err() {
            Err(format!("Opening content failed with: {}", err0).into())
        } else {
            Ok(ReadOnlyContent {
                content: Content {
                    id: id,
                    consumed: false,
                },
            })
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::content::new::test as new_test;
    use crate::storage::Content;
    use cid::Cid;
    pub static CID: &str = "QmbRDNT7TiEKHdav4iZYjLj5Q5QB2BTkKwXeXV3xggQWCR";

    #[test]
    fn open() {
        let cid = Cid::try_from(CID).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let read_only = Content::open(cid).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(read_only.content.id, new_test::ID);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use cid::Cid;

    use crate::{
        errno::{Errno, Error},
        storage::content::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageOpenCid(id: *mut u32, cid_ptr: *const u8) -> Error {
        use super::test;

        let cid_bytes = utils::read_bytes(cid_ptr, 64);
        let err = Cid::try_from(cid_bytes);
        if err.is_err() {
            return Errno::ErrorCap.error();
        }
        let cid = err.unwrap();

        if cid.to_string() != test::CID {
            Errno::ErrorCap.error()
        } else {
            utils::write_u32(id, new_test::ID);
            Errno::ErrorNone.error()
        }
    }
}
