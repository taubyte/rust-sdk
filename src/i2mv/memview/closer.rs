use super::{from_bool, imports, Closer, Error};

fn new_mem_view(data: &[u8], closable: bool, id: *mut u32) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::memoryViewNew(data.as_ptr(), data.len(), from_bool(closable), id)
    }
}

impl Closer {
    pub fn new(data: &[u8], closable: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err = new_mem_view(data, closable, &mut id);
        if err.is_err() {
            Err(format!("creating memory view failed with: {}", err).into())
        } else {
            Ok(Closer { id })
        }
    }

    fn close_unsafe(&self) {
        #[allow(unused_unsafe)]
        unsafe {
            imports::memoryViewClose(self.id)
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.close_unsafe();
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    pub static MEMORY_VIEW_ID: u32 = 1;
    pub static FAKE_DATA: &str = "Hello world";

    use super::Closer;

    #[test]
    fn test_new() {
        let fifo = Closer::new(FAKE_DATA.as_bytes(), true)
            .unwrap_or_else(|err| panic!("writing to  memory view failed with: {}", err));
        assert_eq!(fifo.id, MEMORY_VIEW_ID)
    }

    #[test]
    fn test_close() {
        let mv = Closer::new(FAKE_DATA.as_bytes(), true)
            .unwrap_or_else(|err| panic!("writing to  memory view failed with: {}", err));

        mv.close()
            .unwrap_or_else(|err| panic!("closing memory view failed with: {}", err))
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub mod mock {
    use super::test;
    use crate::{
        errno::{Errno, Error},
        utils::test as utils,
    };

    pub fn memoryViewNew(
        bufPtr: *const u8,
        _bufSize: usize,
        _isCloser: u32,
        idPtr: *mut u32,
    ) -> Error {
        if bufPtr != test::FAKE_DATA.as_bytes().as_ptr() {
            return Errno::ErrorCap.error();
        }
        utils::write_u32(idPtr, test::MEMORY_VIEW_ID);

        Errno::ErrorNone.error()
    }

    pub fn memoryViewClose(id: u32) {
        if id != test::MEMORY_VIEW_ID {
            panic!()
        }
    }
}
