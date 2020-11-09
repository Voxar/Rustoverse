use super::shared;
use serde_json::json;

pub trait WireObject {
    fn to_wire(&self) -> String;
}

impl WireObject for shared::Interaction<T> {
    fn to_wire(&self) -> String {
        json!([
            "interaction",
            self.kind,
            self.sender_entity_id,
            self.receiver_entity_id,
            self.request_id,
            self.body.as_bytes(),
        ]).to_string()
    }
}