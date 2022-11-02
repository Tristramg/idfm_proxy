use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Recieved {
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Broadcast>,
}

#[derive(Serialize, Deserialize)]
pub struct HtmxMessage {
    pub chat_message: String,
}
