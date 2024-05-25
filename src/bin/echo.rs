use nodeforge::{Message, Node, Workload, NOT_SUPPORTED};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum EchoPayload {
    Echo { echo: String },
    EchoOk { echo: String },
    Error { code: usize, text: String },
}

struct EchoNode {}

impl Workload for EchoNode {
    type Payload = EchoPayload;

    fn init() -> Self {
        EchoNode {}
    }

    fn handle_message(&mut self, input: &Message<Self::Payload>) -> Self::Payload {
        match &input.body.payload {
            Self::Payload::Echo { echo } => {
                let echo = echo.clone();
                Self::Payload::EchoOk { echo }
            }
            _ => Self::Payload::Error {
                code: NOT_SUPPORTED,
                text: "Not supported message.".to_string(),
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    _ = Node::<EchoNode>::start()?;
    Ok(())
}
