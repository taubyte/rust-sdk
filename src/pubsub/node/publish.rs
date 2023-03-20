use super::{imports, Channel};
use crate::errno::Error;

impl Channel {
    fn publish_unsafe(&self, data: *const u8, size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::publishToChannel(self.name.as_ptr(), self.name.len(), data, size)
        }
    }

    pub fn publish(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.publish_unsafe(data.as_ptr(), data.len());
        if err0.is_err() {
            Err(format!(
                "Publishing to channel: `{}` failed with: {}",
                self.name, err0
            )
            .into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::pubsub::Channel;
    pub static NAME: &str = "testChannel";
    pub static DATA: &[u8] = b"testData";

    #[test]
    fn channel_publish() {
        let channel = Channel::new(NAME.to_string()).unwrap();
        channel.publish(DATA).unwrap();
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

    pub fn publishToChannel(
        name_ptr: *const u8,
        name_size: usize,
        data_ptr: *const u8,
        data_size: usize,
    ) -> Error {
        let name = utils::read_string(name_ptr, name_size);
        let data = utils::read_bytes(data_ptr, data_size);

        if name != test::NAME || data != test::DATA {
            Errno::ErrorCap.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}
