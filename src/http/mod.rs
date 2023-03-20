pub use event::Event;

mod event {
    mod body;
    mod headers;
    mod host;
    mod imports;
    mod method;
    mod path;
    mod query;
    mod r#return;
    mod user_agent;
    mod write;

    #[derive(Copy, Clone)]
    pub struct Event {
        pub event: u32,
    }

    pub struct EventBody {
        pub consumed: bool,
        pub event: u32,
    }

    pub struct EventHeaders {
        pub event: u32,
    }

    pub struct EventQueries {
        pub event: u32,
    }
}

pub use client::Client;

mod client {
    mod imports;
    mod methods;
    mod new;
    mod request;
    mod response;
    mod send;

    pub struct Client {
        pub id: u32,
    }
}
