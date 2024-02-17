use serde::Serialize;
use serde::Deserialize;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation{
    pub messages: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation { 
            messages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message{
    pub sender: bool,
    pub msg: String
}

