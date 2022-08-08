use release_testing_model::{Channel, Message};

pub fn get_message() -> Message {
    Message {
        content: String::from("message content"),
    }
}

pub fn get_channel() -> Message {
    Channel {
        name: String::from("channel name"),
    }
}

