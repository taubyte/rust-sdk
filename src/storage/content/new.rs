use crate::errno::Error;

use super::{imports, Content, ReadWriteContent};

impl Content {
    fn new_unsafe(id: *mut u32) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageNewContent(id)
        }
    }

    pub fn new() -> Result<ReadWriteContent, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err0 = Content::new_unsafe(&mut id);
        if err0.is_err() {
            Err(format!("Creating new content failed with: {}", err0).into())
        } else {
            Ok(ReadWriteContent {
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
    use crate::storage::Content;

    pub static ID: u32 = 4;

    #[test]
    fn new() {
        let read_write = Content::new().unwrap_or_else(|err| {
            panic!("{}", err);
        });
        assert_eq!(read_write.content.id, ID);
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

    pub fn storageNewContent(id: *mut u32) -> Error {
        utils::write_u32(id, test::ID);
        Errno::ErrorNone.error()
    }
}
