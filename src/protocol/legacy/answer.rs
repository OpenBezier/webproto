use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Answer {
    pub event_id: String,
    pub data: Vec<u8>,
}

impl Answer {
    pub fn encode(data: &impl Serialize, event_id: String) -> anyhow::Result<Self> {
        let mut out_data = Vec::new();
        data.serialize(&mut rmp_serde::Serializer::new(&mut out_data))
            .map_err(|e| anyhow::anyhow!("encode data to Answer with err: {:?}", e))?;
        anyhow::Ok(Answer {
            event_id: event_id,
            data: out_data,
        })
    }

    pub fn decode<'a, T>(&'a self) -> anyhow::Result<(T, String)>
    where
        T: Deserialize<'a>,
    {
        let data = rmp_serde::decode::from_slice::<T>(&self.data)
            .map_err(|e| anyhow::anyhow!("decode data from Answer with err: {:?}", e))?;
        anyhow::Ok((data, self.event_id.clone()))
    }

    pub fn to_vec(&self) -> anyhow::Result<Vec<u8>> {
        let mut out_data = Vec::new();
        self.serialize(&mut rmp_serde::Serializer::new(&mut out_data))
            .map_err(|e| anyhow::anyhow!("to_vec with err: {:?}", e))?;
        anyhow::Ok(out_data)
    }

    pub fn from_vec(data: &Vec<u8>) -> anyhow::Result<Self> {
        let answer = rmp_serde::decode::from_slice::<Answer>(&data)
            .map_err(|e| anyhow::anyhow!("from_vec with err: {:?}", e))?;
        anyhow::Ok(answer)
    }
}
