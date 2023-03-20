use super::{imports, Event, EventBody};
use crate::errno::Errno;

impl Event {
    pub fn body(&self) -> EventBody {
        EventBody {
            consumed: false,
            event: (self.event),
        }
    }
}

impl std::io::Read for EventBody {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.consumed {
            return Ok(0);
        }

        let mut n = 0;

        let err0 =
            unsafe { imports::readHttpEventBody(self.event, buf.as_mut_ptr(), buf.len(), &mut n) };
        if err0.is_errno(Errno::ErrorEOF) {
            // TODO, should we close here?

            self.consumed = true;
            Ok(n as usize)
        } else if err0.ok() {
            Ok(n as usize)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("reading body failed with: {}", err0),
            ))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    use std::io::Read;
    pub static EXPECTED_BODY: &str = "Hello, world!";
    pub static EXPECTED_ID: u32 = 0;

    #[test]
    fn test_body() {
        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let mut body = http.body();
        let mut buffer = String::new();
        body.read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, EXPECTED_BODY);
    }
}
