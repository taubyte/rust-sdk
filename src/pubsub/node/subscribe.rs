use super::{imports, Channel};
use crate::errno::Error;

impl Channel {
    fn subscribe_unsafe(&self) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::setSubscriptionChannel(self.name.as_ptr(), self.name.len())
        }
    }

    pub fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.subscribe_unsafe();
        if err0.is_err() {
            Err(format!(
                "Subscribing to channel: `{}` failed with: {}",
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

    #[test]
    fn subscribe() {
        let channel = Channel::new(NAME.to_string()).unwrap();
        channel.subscribe().unwrap();
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

    pub fn setSubscriptionChannel(name_ptr: *const u8, name_size: usize) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        if name != test::NAME {
            Errno::ErrorCap.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}
