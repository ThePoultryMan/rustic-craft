use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    name: String,
}

impl Item {
    pub fn new(identifier: &str) -> Self {
        Self {
            name: identifier.to_owned(),
        }
    }
}