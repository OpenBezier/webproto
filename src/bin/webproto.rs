use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use webproto::{decode_message, ClientCommand, Indication, Message};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TestData {
    pub msg: String,
}

fn main() -> anyhow::Result<()> {
    let test_data = TestData {
        msg: "one test".into(),
    };

    let command = ClientCommand::<TestData>::encode(&test_data, "101".into())?;
    // let command = Indication::to_msg_vec(&test_data)?;
    println!("{:?}", command);

    let message = decode_message::<TestData>(&command)?;
    println!("{:?}", message);

    match message {
        Message::ClientCommand(cmd) => {
            println!("client command {:?}", cmd);
        }
        Message::ServerCommand(cmd) => {
            println!("server command {:?}", cmd);
        }
        Message::Indication(ind) => {
            println!("indication {:?}", ind);
        }
    }

    anyhow::Ok(())
}
