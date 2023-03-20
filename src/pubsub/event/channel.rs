use super::{imports, Event};
use crate::{errno::Error, pubsub::node::Channel};

impl Event {
    fn channel_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getMessageChannelSize(self.event, size)
        }
    }

    fn channel_unsafe(&self, buf_ptr: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getMessageChannel(self.event, buf_ptr)
        }
    }

    pub fn channel(&self) -> Result<Channel, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.channel_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!(
                "Getting channel size for event: `{}` failed with: {}",
                self.event, err0
            )
            .into());
        }
        if size == 0 {
            return Ok(Channel::new("".to_string())?);
        }

        let mut buf = vec![0u8; size];
        let err0 = self.channel_unsafe(buf.as_mut_ptr());
        if err0.is_err() {
            Err(format!(
                "Getting channel for event: `{}` failed with: {}",
                self.event, err0
            )
            .into())
        } else {
            Ok(Channel::new(String::from_utf8(buf)?)?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::pubsub::Event;
    pub static ID: u32 = 3;
    pub static CHANNEL: &str = "testChannel";

    #[test]
    fn event_channel() {
        let event = Event { event: ID };
        let channel = event.channel().unwrap();
        assert_eq!(channel.name(), CHANNEL);
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

    pub fn getMessageChannelSize(event: u32, size: *mut usize) -> Error {
        if event != test::ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(size, test::CHANNEL.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn getMessageChannel(event: u32, buf_ptr: *mut u8) -> Error {
        if event != test::ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_string(buf_ptr, test::CHANNEL);
            Errno::ErrorNone.error()
        }
    }
}
