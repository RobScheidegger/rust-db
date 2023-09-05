pub mod server {
    pub mod node;
    pub mod database;
}

pub mod client { }

pub mod shared {
    pub mod messages {
        pub mod insert_message;
    }

    pub mod message;
}