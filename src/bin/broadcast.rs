use std::collections::HashMap;

use nodeforge::Workload;
use serde::{Deserialize, Serialize};

type MessageType = usize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum BroadcastPayload {
    Broadcast {
        message: MessageType,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<MessageType>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Error {
        code: usize,
        text: String,
    },
}

#[derive(Debug, Default)]
struct BroadcastNode {
    messages: Vec<MessageType>,
    topology: HashMap<String, Vec<String>>,
}

impl Workload for BroadcastNode {
    type Payload = BroadcastPayload;

    fn init() -> Self {
        Self::default()
    }

    fn handle_message(&mut self, input: &nodeforge::Message<Self::Payload>) -> Self::Payload {
        match &input.body.payload {
            Self::Payload::Broadcast { message } => {
                self.messages.push(*message);
                Self::Payload::BroadcastOk
            }
            Self::Payload::Read => Self::Payload::ReadOk {
                messages: self.messages.clone(),
            },
            Self::Payload::Topology { topology } => {
                self.topology = topology.clone();
                Self::Payload::TopologyOk
            }
            _ => Self::Payload::Error {
                code: nodeforge::NOT_SUPPORTED,
                text: "Not supported message.".to_string(),
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let _ = nodeforge::Node::<BroadcastNode>::start()?;
    Ok(())
}
