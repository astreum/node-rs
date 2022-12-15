use std::error::Error;

use crate::relay::Message;
use crate::relay::Topic;

impl Message {

    pub fn new(body: &[u8], topic: &Topic) -> Message {
        Message { body: body.to_vec(), topic: topic.clone() }
    }

}

impl Into<Vec<u8>> for &Message {

    fn into(self) -> Vec<u8> {
        
        astro_format::encode(&[
            &self.body
        ])

    }

}

impl TryFrom<&[u8]> for Message {
    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

        let decoded_message = astro_format::decode(value)?;

        if decoded_message.len() == 2 {

            Ok(Message::new(
                decoded_message[0],
                &(decoded_message[1].try_into()?)
            ))

        } else {

            Err("Internal error!")?

        }
    }
}
