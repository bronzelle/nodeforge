use nodeforge::{Message, Node, Workload, NOT_SUPPORTED};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum GeneratePayload {
    Generate,
    GenerateOk { id: String },
    Error { code: usize, text: String },
}

struct GenerateNode {}

impl Workload for GenerateNode {
    type Payload = GeneratePayload;

    fn init() -> Self {
        GenerateNode {}
    }

    fn handle_message(&mut self, input: &Message<Self::Payload>) -> Self::Payload {
        match input.body.payload {
            Self::Payload::Generate => {
                let id = Uuid::new_v4().to_string();
                Self::Payload::GenerateOk { id }
            }
            _ => Self::Payload::Error {
                code: NOT_SUPPORTED,
                text: "Not supported message.".to_string(),
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let _ = Node::<GenerateNode>::start()?;
    Ok(())
}
