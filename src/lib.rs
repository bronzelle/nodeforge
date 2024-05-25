use serde::{Deserialize, Serialize};

pub const _TIMEOUT: usize = 0;
pub const _NODE_NOT_FOUND: usize = 1;
pub const NOT_SUPPORTED: usize = 10;
pub const _TEMPORARILY_UNAVAILABLE: usize = 11;
pub const _MALFORMED_REQUEST: usize = 12;
pub const _CRASH: usize = 13;
pub const _ABORT: usize = 14;
pub const _KEY_DOES_NOT_EXIST: usize = 20;
pub const _KEY_ALREADY_EXISTS: usize = 21;
pub const _PRECONDITION_FAILED: usize = 22;
pub const _TXN_CONFLICT: usize = 30;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message<Payload> {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    pub body: Body<Payload>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Body<Payload> {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

pub struct Node<T: Workload> {
    pub id: String,
    pub nodes: Vec<String>,
    msg_id: usize,
    specific: T,
}

pub trait Workload {
    type Payload: Deserialize<'static> + Serialize;
    fn init() -> Self;
    fn handle_message(&mut self, input: &Message<Self::Payload>) -> Self::Payload;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum InitPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

impl<T: Workload> Node<T> {
    pub fn start() -> anyhow::Result<()> {
        let input = std::io::stdin().lock();
        let output = std::io::stdout().lock();
        let mut reader =
            serde_json::Deserializer::from_reader(input).into_iter::<Message<InitPayload>>();
        let mut writer = serde_json::Serializer::new(output);

        let Some(Ok(init)) = reader.next() else {
            return Err(anyhow::anyhow!("Failed to read init message"));
        };
        let InitPayload::Init { node_id, node_ids } = init.body.payload.clone() else {
            return Err(anyhow::anyhow!("Failed to parse init message"));
        };
        let mut this = Self {
            id: node_id,
            nodes: node_ids,
            msg_id: 0,
            specific: T::init(),
        };
        this.reply(&init, InitPayload::InitOk)
            .serialize(&mut writer)
            .map_err(|e| e)?;
        // flush the buffer
        println!("");

        drop(reader);

        let input = std::io::stdin().lock();
        let reader =
            serde_json::Deserializer::from_reader(input).into_iter::<Message<T::Payload>>();

        for message in reader {
            let message = message?;
            let reply_payload = this.specific.handle_message(&message);
            let reply = this.reply(&message, reply_payload);

            reply.serialize(&mut writer)?;
            println!("");
        }
        Ok(())
    }

    fn reply<P, Q>(&mut self, input: &Message<P>, payload: Q) -> Message<Q>
    where
        P: Deserialize<'static>,
        Q: Serialize,
    {
        let message = Message {
            src: input.dst.clone(),
            dst: input.src.clone(),
            body: Body {
                msg_id: Some(self.msg_id),
                in_reply_to: input.body.msg_id,
                payload,
            },
        };
        self.msg_id += 1;
        message
    }
}
