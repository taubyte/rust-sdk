pub use {event::Event, node::Channel};

mod event {
    #[derive(Copy, Clone)]
    pub struct Event {
        pub event: u32,
    }

    mod channel;
    mod data;

    #[warn(unused_imports)]
    #[cfg(not(test))]
    mod imports;

    #[cfg(test)]
    mod imports {
        pub use super::channel::mock::*;
        pub use super::data::mock::*;
    }
}

mod node {
    pub struct Channel {
        name: String,
    }

    pub type WebSocket = Channel;
    mod channel;
    mod publish;
    mod socket;
    mod subscribe;

    #[warn(unused_imports)]
    #[cfg(not(test))]
    mod imports;

    #[cfg(test)]
    mod imports {
        pub use super::publish::mock::*;
        pub use super::socket::mock::*;
        pub use super::subscribe::mock::*;
    }
}
