use crate::{http, pubsub};

use super::{imports, Error, Event};

#[derive(PartialEq, Debug)]
pub enum EventType {
    EventTypeUndefined,
    EventTypeHttp,
    EventTypePubsub,
    EventTypeP2P,
}

impl Event {
    fn get_event_type(&self, id: *mut u32) {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getEventType(self.event, id)
        }
    }

    pub fn event_type(&self) -> EventType {
        let mut id = 0;
        self.get_event_type(&mut id);
        match id {
            0 => EventType::EventTypeUndefined,
            1 => EventType::EventTypeHttp,
            2 => EventType::EventTypePubsub,
            3 => EventType::EventTypeP2P,
            _ => EventType::EventTypeUndefined,
        }
    }

    pub fn http(&self) -> Result<http::Event, Box<dyn Error>> {
        if self.event_type() == EventType::EventTypeHttp {
            Ok(http::Event {
                event: (self.event),
            })
        } else {
            Err("Not an HTTP Event".into())
        }
    }

    pub fn pubsub(&self) -> Result<pubsub::Event, Box<dyn Error>> {
        if self.event_type() == EventType::EventTypePubsub {
            Ok(pubsub::Event {
                event: (self.event),
            })
        } else {
            Err("Not a PubSub Event".into())
        }
    }
}

#[cfg(test)]
pub mod test {
    pub static mut EVENT_TYPE: &u32 = &0;

    #[test]
    fn test_event_type() {
        use super::{Event, EventType};
        let event = Event { event: 0 };

        assert_eq!(event.event_type(), EventType::EventTypeUndefined);

        let http = event.http();
        if !http.is_err() {
            panic!("Expected error, got: {:?}", http.err());
        }

        // Set to event type http
        unsafe {
            EVENT_TYPE = &(EventType::EventTypeHttp as u32);
        }

        event.http().unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;
    use crate::utils::test as utils;

    pub fn getEventType(_baseEvent: u32, id: *mut u32) {
        unsafe { utils::write_u32(id, *test::EVENT_TYPE as u32) }
    }
}
