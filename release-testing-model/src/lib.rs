pub struct Channel {
    pub name: String,
}

pub struct Message {
    pub content: String,
}

pub struct Guild {
    pub channels: Vec<Channel>,
    pub name: String,
}
