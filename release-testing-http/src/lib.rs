use release_testing_model::Message;

pub fn get_message() -> Message {
    Message {
        content: String::from("message content"),
    }
}

