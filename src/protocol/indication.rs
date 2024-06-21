use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Indication<F> {
    pub data: F,
}

impl<F> Indication<F> {
    pub fn encode(data: impl Serialize) -> anyhow::Result<Vec<u8>> {
        let msg = Message::Indication(Indication { data: data });
        let msg_data = rmp_serde::to_vec_named(&msg)
            .map_err(|e| anyhow::anyhow!("encode data to vec_u8 with err: {:?}", e))?;
        anyhow::Ok(msg_data)
    }
}
