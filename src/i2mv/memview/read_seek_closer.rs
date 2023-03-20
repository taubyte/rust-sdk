use super::{imports, to_bool, Error, ReadSeekCloser};
use std::io::{Read, Seek};

fn open_memory_view(id: u32, is_closable: *mut u32, size: *mut usize) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::memoryViewOpen(id, is_closable, size)
    }
}

impl ReadSeekCloser {
    pub fn open(id: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut is_closable = 0;
        let mut size = 0;
        let err = open_memory_view(id, &mut is_closable, &mut size);
        if err.is_err() {
            Err(format!("open failed with: {}", err).into())
        } else {
            Ok(ReadSeekCloser {
                id: id,
                size: size,
                offset: 0,
                closable: to_bool(is_closable),
            })
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.closable {
            self.close_unsafe();
            Ok(())
        } else {
            Err(format!("fifo `{}` is not closable", self.id).into())
        }
    }

    fn close_unsafe(&self) {
        #[allow(unused_unsafe)]
        unsafe {
            imports::memoryViewClose(self.id)
        }
    }

    fn read_unsafe(&self, buffer: &mut [u8], n_ptr: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::memoryViewRead(
                self.id,
                self.offset as u32,
                buffer.len(),
                buffer.as_mut_ptr(),
                n_ptr,
            )
        }
    }
}

impl Seek for ReadSeekCloser {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::result::Result<u64, std::io::Error> {
        let offset: i64;
        match pos {
            std::io::SeekFrom::Start(start) => {
                offset = start as i64;
            }
            std::io::SeekFrom::Current(current) => {
                offset = current as i64 + self.offset as i64;
            }
            std::io::SeekFrom::End(end) => {
                offset = end as i64 + self.size as i64;
            }
        };
        if offset < 0 {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cannot seek before start",
            ))
        } else if offset > self.size as i64 {
            self.offset = self.size as i64;
            Ok(self.offset as u64)
        } else {
            self.offset = offset;
            Ok(self.offset as u64)
        }
    }
}

impl Read for ReadSeekCloser {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        if buf.len() == 0 {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "cannot read to nil bytes",
            ))
        } else if self.offset >= self.size as i64 {
            Ok(0)
        } else {
            let mut n: usize = 0;
            let err0 = self.read_unsafe(buf, &mut n);
            if err0.is_err() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Reading memory view failed with: {}", err0),
                ))
            } else {
                self.offset = self.offset + n as i64;
                Ok(n as usize)
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::ReadSeekCloser;
    pub static MEMORY_VIEW_IS_OPEN: u32 = 1;
    use crate::i2mv::memview::closer::test as c_test;
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    fn test_open() {
        let mv = ReadSeekCloser::open(1)
            .unwrap_or_else(|err| panic!("opening memory view failed with: {}", err));

        assert_eq!(mv.id, c_test::MEMORY_VIEW_ID);
        assert_eq!(mv.size as usize, c_test::FAKE_DATA.as_bytes().len());
        assert_eq!(mv.offset, 0);
    }

    #[test]
    fn test_read() {
        let mut mv = ReadSeekCloser::open(1)
            .unwrap_or_else(|err| panic!("opening memory view failed with: {}", err));

        let mut buffer = String::new();
        mv.read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, c_test::FAKE_DATA)
    }

    #[test]
    fn test_seek() {
        let mut mv = ReadSeekCloser::open(1)
            .unwrap_or_else(|err| panic!("opening memory view failed with: {}", err));

        let mut n = mv
            .seek(SeekFrom::Start(10))
            .unwrap_or_else(|err| panic!("seeking failed with: {}", err));
        assert_eq!(n, 10);

        n = mv
            .seek(SeekFrom::Current(1))
            .unwrap_or_else(|err| panic!("seeking failed with: {}", err));
        assert_eq!(n, 11);

        n = mv
            .seek(SeekFrom::End(10))
            .unwrap_or_else(|err| panic!("seeking failed with: {}", err));
        assert_eq!(n, 11);

        n = mv
            .seek(SeekFrom::End(-10))
            .unwrap_or_else(|err| panic!("seeking failed with: {}", err));
        assert_eq!(n, 1);
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub mod mock {
    use super::test;
    use crate::{
        errno::{Errno, Error},
        i2mv::memview::closer::test as c_test,
        utils::test as utils,
    };

    pub fn memoryViewOpen(id: u32, isClosablePtr: *mut u32, size: *mut usize) -> Error {
        if id != c_test::MEMORY_VIEW_ID {
            return Errno::ErrorCap.error();
        }

        utils::write_u32(isClosablePtr, test::MEMORY_VIEW_IS_OPEN);
        utils::write_usize(size, c_test::FAKE_DATA.as_bytes().len());

        Errno::ErrorNone.error()
    }

    pub fn memoryViewRead(
        id: u32,
        off_set: u32,
        _count: usize,
        bufPtr: *mut u8,
        nPtr: *mut usize,
    ) -> Error {
        if id != c_test::MEMORY_VIEW_ID {
            return Errno::ErrorCap.error();
        }
        let mut count = _count;
        let dataBytes = c_test::FAKE_DATA.as_bytes();
        let dataSize = dataBytes.len();

        if off_set as usize >= dataSize {
            return Errno::ErrorAddressOutOfMemory.error();
        }

        if dataSize < off_set as usize + count {
            count = dataSize - off_set as usize
        }

        utils::write_bytes(bufPtr, dataBytes);
        utils::write_usize(nPtr, count);

        Errno::ErrorNone.error()
    }
}
