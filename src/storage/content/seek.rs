use std::io::Seek;

use crate::storage::Content;

use super::{imports, ReadOnlyContent, ReadWriteContent};

// Implement seek
impl Seek for Content {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::result::Result<u64, std::io::Error> {
        self.consumed = false;

        let offset: i64;
        let whence: i32 = match pos {
            std::io::SeekFrom::Start(start) => {
                offset = start as i64;
                0
            }
            std::io::SeekFrom::End(end) => {
                offset = end as i64;
                2
            }
            std::io::SeekFrom::Current(current) => {
                offset = current as i64;
                1
            }
        };
        let mut offset_ptr: i32 = 0;
        #[allow(unused_unsafe)]
        let err0 = unsafe { imports::contentSeekFile(self.id, offset, whence, &mut offset_ptr) };
        if err0.is_err() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Seeking content failed with: {}", err0),
            ))
        } else {
            Ok(offset_ptr as u64)
        }
    }
}

impl std::io::Seek for ReadWriteContent {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::result::Result<u64, std::io::Error> {
        self.content.seek(pos)
    }
}

impl std::io::Seek for ReadOnlyContent {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::result::Result<u64, std::io::Error> {
        self.content.seek(pos)
    }
}

#[cfg(test)]
pub mod test {
    use std::io::Seek;

    use crate::storage::Content;

    #[test]
    fn seek() {
        let mut content = Content::new().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        content
            .seek(std::io::SeekFrom::Start(0))
            .unwrap_or_else(|err| {
                panic!("{}", err);
            });
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        storage::content::new::test as new_test,
    };

    pub fn contentSeekFile(id: u32, offset: i64, whence: i32, offset_ptr: *mut i32) -> Error {
        assert_eq!(id, new_test::ID);
        assert_eq!(offset, 0);
        assert_eq!(whence, 0);
        assert_ne!(offset_ptr, std::ptr::null_mut());

        let offset_ptr = unsafe { offset_ptr.as_mut().unwrap() };
        *offset_ptr = 0;

        Errno::ErrorNone.error()
    }
}
