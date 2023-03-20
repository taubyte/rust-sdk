use super::{imports, to_bool, Errno, Error, ReadCloser};

fn open_fifo(id: u32, is_closable: *mut u32) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::fifoIsCloser(id, is_closable)
    }
}

impl ReadCloser {
    fn close_fifo(&self) {
        #[allow(unused_unsafe)]
        unsafe {
            imports::fifoClose(self.id)
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.closable {
            return Err(format!("fifo `{}` is not closable", self.id).into());
        } else {
            self.close_fifo();
            Ok(())
        }
    }

    pub fn open(id: u32) -> Result<ReadCloser, Box<dyn std::error::Error>> {
        let mut is_closable = 0;

        let err0 = open_fifo(id, &mut is_closable);
        if err0.is_err() {
            Err(format!("open failed with: {}", err0).into())
        } else {
            Ok(ReadCloser {
                id: id,
                closable: to_bool(is_closable),
            })
        }
    }
}

impl std::io::Read for ReadCloser {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let mut n: usize = 0;
        for byte in buffer {
            let err0 = unsafe { imports::fifoPop(self.id, byte as *mut u8) };
            if err0.is_errno(Errno::ErrorEOF) {
                return Ok(n);
            } else if err0.is_err() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("fifoPop failed with: {}", err0),
                ));
            }

            n += 1
        }
        Ok(n)
    }
}
#[cfg(test)]
pub mod test {
    use super::ReadCloser;
    use crate::i2mv::fifo::write_closer::test as wc_test;
    use std::io::Read;

    pub static FIFO_IS_CLOSER: u32 = 1;
    pub static mut IDX: usize = 0;

    #[test]
    fn test_open() {
        let fifo = ReadCloser::open(wc_test::FIFO_ID)
            .unwrap_or_else(|err| panic!("opening fifo failed with: {}", err));

        assert_eq!(fifo.id, wc_test::FIFO_ID);
        assert_eq!(fifo.closable, true);
    }

    #[test]
    fn test_close() {
        let fifo = ReadCloser::open(wc_test::FIFO_ID)
            .unwrap_or_else(|err| panic!("opening fifo failed with: {}", err));

        fifo.close().unwrap_or_else(|err| {
            panic!("closing fifo failed with: {}", err);
        });
    }

    #[test]
    fn test_read() {
        let mut fifo = ReadCloser::open(wc_test::FIFO_ID)
            .unwrap_or_else(|err| panic!("opening fifo failed with: {}", err));

        let mut buffer = String::new();
        fifo.read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, wc_test::FAKE_DATA)
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub mod mock {
    use super::{test::{self, IDX}, Error};
    use crate::{errno::Errno, i2mv::fifo::write_closer::test as wc_test, utils::test as utils};



    pub fn fifoIsCloser(id: u32, isCloser: *mut u32) -> Error {
        if id != wc_test::FIFO_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_u32(isCloser, test::FIFO_IS_CLOSER);
            Errno::ErrorNone.error()
        }
    }

    pub unsafe fn fifoPop(id: u32, bufPtr: *mut u8) -> Error {
        if id != wc_test::FIFO_ID {
            Errno::ErrorCap.error()
        } else {
            let data = wc_test::FAKE_DATA.as_bytes();
            if test::IDX == data.len() {
                Errno::ErrorEOF.error()
            } else {
                utils::write_u8(bufPtr, data[IDX]);
                IDX += 1;
                Errno::ErrorNone.error()
            }
        }
    }
}
