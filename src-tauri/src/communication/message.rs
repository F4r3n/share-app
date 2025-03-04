#[derive(Clone, serde::Serialize)]
pub struct ResponseMessage {
    pub kind: u16,
    pub content: Vec<String>,
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub content: String,
    pub nick_name: String,
    pub command: String,
    pub channel: String,
    pub response: Option<Vec<String>>,
}

impl Default for Payload {
    fn default() -> Payload {
        Payload {
            content: "".into(),
            nick_name: "".into(),
            command: "".into(),
            channel: "".into(),
            response: None,
        }
    }
}
