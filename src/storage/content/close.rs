use crate::{errno::Error, storage::Content};

use super::{imports, ReadOnlyContent, ReadWriteContent};

impl Content {
    fn close_unsafe(&self) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::contentCloseFile(self.id)
        }
    }

    fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.close_unsafe();
        if err0.is_err() {
            Err(format!("Closing content failed with: {}", err0).into())
        } else {
            Ok(())
        }
    }
}

impl ReadWriteContent {
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.content.close()
    }
}

impl ReadOnlyContent {
    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.content.close()
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::content::open::test as open_test;
    use crate::storage::Content;
    use cid::Cid;

    #[test]
    fn close_read_write() {
        let read_write = Content::new().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        read_write.close().unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    #[test]
    fn close_read_only() {
        let cid = Cid::try_from(open_test::CID).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let read_only = Content::open(cid).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        read_only.close().unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::storage::content::new::test as new_test;

    use crate::errno::{Errno, Error};

    pub fn contentCloseFile(id: u32) -> Error {
        assert_eq!(id, new_test::ID);
        Errno::ErrorNone.error()
    }
}
