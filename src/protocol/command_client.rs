use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClientCommand<F> {
    pub event_id: String,
    pub command: F,
}

impl<F> ClientCommand<F> {
    pub fn encode(data: impl Serialize, event_id: String) -> anyhow::Result<Vec<u8>> {
        let msg = Message::ClientCommand(ClientCommand {
            event_id: event_id.clone(),
            command: data,
        });
        // let msg_data = rmp_serde::to_vec(&msg)
        //     .map_err(|e| anyhow::anyhow!("encode data to vec_u8 with err: {:?}", e))?;
        let msg_data = rmp_serde::to_vec_named(&msg)
            .map_err(|e| anyhow::anyhow!("encode data to vec_u8 with err: {:?}", e))?;
        anyhow::Ok(msg_data)
    }
}
