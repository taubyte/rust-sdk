use super::{imports, Channel, WebSocket};
use crate::errno::Error;

impl Channel {
    pub fn web_socket(&self) -> WebSocket {
        WebSocket { name: self.name() }
    }
}

impl WebSocket {
    pub fn url_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getWebSocketURLSize(self.name.as_ptr(), self.name.len(), size)
        }
    }

    pub fn url_unsafe(&self, buf_ptr: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getWebSocketURL(self.name.as_ptr(), self.name.len(), buf_ptr)
        }
    }

    pub fn url(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.url_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!(
                "Getting url size for channel: `{}` failed with: {}",
                self.name, err0
            )
            .into());
        }
        if size == 0 {
            return Ok(String::new());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.url_unsafe(buf.as_mut_ptr());
        if err0.is_err() {
            Err(format!(
                "Getting url for channel: `{}` failed with: {}",
                self.name, err0
            )
            .into())
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::pubsub::Channel;
    pub static NAME: &str = "testChannel";
    pub static URL: &str = "testUrl";

    #[test]
    fn web_socket() {
        let channel = Channel::new(NAME.to_string()).unwrap();
        let web_socket = channel.web_socket();
        assert_eq!(web_socket.name(), NAME);

        let url = web_socket.url().unwrap();
        assert_eq!(url, URL);
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

    pub fn getWebSocketURLSize(name_ptr: *const u8, name_size: usize, size: *mut usize) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        if name != test::NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(size, test::URL.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn getWebSocketURL(name_ptr: *const u8, name_size: usize, buf_ptr: *mut u8) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        if name != test::NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_string(buf_ptr, test::URL);
            Errno::ErrorNone.error()
        }
    }
}
