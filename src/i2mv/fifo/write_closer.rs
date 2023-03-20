use super::{from_bool, imports, Error, WriteCloser};
use std::io::Write;

fn new_fifo(closable: bool) -> u32 {
    unsafe { imports::fifoNew(from_bool(closable)) }
}

impl Write for WriteCloser {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        let mut n = 0;
        let mut err0 = Error { id: 0 };
        for byte in buf {
            err0 = self.fifo_push(*byte);
            if err0.is_err() {
                break;
            } else {
                n += 1
            }
        }

        if err0.is_err() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("pushing byte to FIFO failed with: {}", err0),
            ))
        } else {
            Ok(n)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // TODO: implement
        Ok(())
    }
}

impl WriteCloser {
    pub fn new(closable: bool) -> Self {
        return WriteCloser {
            id: new_fifo(closable),
        };
    }

    fn fifo_push(&self, buf: u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::fifoPush(self.id, buf)
        }
    }

    fn close_unsafe(&self) {
        #[allow(unused_unsafe)]
        unsafe {
            imports::fifoClose(self.id)
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.close_unsafe();
        Ok(())
    }
}
#[cfg(test)]
pub mod test {
    pub static FIFO_ID: u32 = 1;
    pub static FAKE_DATA: &str = "Hello world";

    use super::WriteCloser;
    use std::io::Write;

    #[test]
    fn test_new() {
        let fifo = WriteCloser::new(true);
        assert_eq!(fifo.id, FIFO_ID)
    }

    #[test]
    fn test_write() {
        let mut fifo = WriteCloser::new(true);
        let n = fifo
            .write(FAKE_DATA.as_bytes())
            .unwrap_or_else(|err| panic!("writing to fifo failed with: {}", err));

        assert_eq!(n, FAKE_DATA.as_bytes().len());
    }

    #[test]
    fn test_close() {
        let fifo = WriteCloser::new(true);

        fifo.close().unwrap_or_else(|err| {
            panic!("closing fifo failed with: {}", err);
        });
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub mod mock {
    use super::{test, Error};
    use crate::errno::Errno;

    pub unsafe fn fifoNew(_closable: u32) -> u32 {
        return test::FIFO_ID;
    }

    pub fn fifoClose(id: u32) {
        if id != test::FIFO_ID {
            panic!()
        }
    }

    pub fn fifoPush(id: u32, buf: u8) -> Error {
        if id != test::FIFO_ID {
            Errno::ErrorCap.error()
        } else {
            let mut exists: bool = false;
            for byte in test::FAKE_DATA.as_bytes() {
                if *byte == buf {
                    exists = true
                }
            }

            if !exists {
                Errno::ErrorFifoDatatypeInvalid.error()
            } else {
                Errno::ErrorNone.error()
            }
        }
    }
}
