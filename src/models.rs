use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkPayload {
    IDENTIFY(AuthorIdentity),
    MESSAGE(MessagePayload)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    pub content: String,
    pub author: AuthorIdentity
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorIdentity {
    pub nickname: String
}
