pub mod command_client;
pub mod indication;
pub use command_client::*;
pub use indication::*;
pub mod command_server;
pub use command_server::*;

// pub mod command_exit;
// pub use command_exit::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Message<F> {
    Indication(Indication<F>),
    ClientCommand(ClientCommand<F>),
    ServerCommand(ServerCommand<F>),
    // ForceExit(ForceExit<F>),
}

pub fn decode_message<'a, T>(data: &'a Vec<u8>) -> anyhow::Result<Message<T>>
where
    T: Deserialize<'a>,
{
    let data = rmp_serde::decode::from_slice::<Message<T>>(data)
        .map_err(|e| anyhow::anyhow!("decode data from vec_u8 with err: {:?}", e))?;
    anyhow::Ok(data)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MockMessage {
    pub msg: String,
}

impl MockMessage {
    pub fn mock_request() -> Self {
        MockMessage {
            msg: "reqeust".into(),
        }
    }

    pub fn mock_response() -> Self {
        MockMessage {
            msg: "response".into(),
        }
    }
}

// impl Drop for MockMessage {
//     fn drop(&mut self) {
//         println!("drop mock message");
//     }
// }
