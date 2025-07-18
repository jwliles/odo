use std::time::Instant;

pub struct StatusMessage {
    pub text: String,
    pub time: Instant,
}

impl StatusMessage {
    pub fn from<T: Into<String>>(message: T) -> Self {
        Self {
            time: Instant::now(),
            text: message.into(),
        }
    }
}