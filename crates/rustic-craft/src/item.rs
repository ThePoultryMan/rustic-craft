use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    identifier: String,
}

impl Item {
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
        }
    }
}