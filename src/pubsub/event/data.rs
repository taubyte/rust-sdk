use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn data_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getMessageDataSize(self.event, size)
        }
    }

    fn data_unsafe(&self, buf_ptr: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getMessageData(self.event, buf_ptr)
        }
    }

    pub fn data(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.data_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!(
                "Getting data size for event: `{}` failed with: {}",
                self.event, err0
            )
            .into());
        }
        if size == 0 {
            return Ok(Vec::new());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.data_unsafe(buf.as_mut_ptr());
        if err0.is_err() {
            Err(format!(
                "Getting data for event: `{}` failed with: {}",
                self.event, err0
            )
            .into())
        } else {
            Ok(buf)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::pubsub::Event;
    pub static ID: u32 = 6;
    pub static DATA: &[u8] = b"testData";

    #[test]
    fn event_data() {
        let event = Event { event: ID };
        let data = event.data().unwrap();
        assert_eq!(data, DATA);
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

    pub fn getMessageDataSize(event: u32, size: *mut usize) -> Error {
        if event != test::ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(size, test::DATA.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn getMessageData(event: u32, buf_ptr: *mut u8) -> Error {
        if event != test::ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_bytes(buf_ptr, test::DATA);
            Errno::ErrorNone.error()
        }
    }
}
