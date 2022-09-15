use serde::{Deserialize, Serialize};

use std::{fs::File, io::Error};

#[derive(Deserialize, Serialize)]
pub struct MinecraftMod {
    mod_id: String,
}

impl MinecraftMod {
    pub fn new(mod_id: &str) -> Self {
        Self {
            mod_id: mod_id.to_owned(),
        }
    }

    fn get_mod_id(&self) -> &str {
        &self.mod_id
    }
}

pub fn create_mod_json_file(minecraft_mod: MinecraftMod) -> Result<(), Error> {
    serde_json::to_writer(File::create("./".to_owned() + minecraft_mod.get_mod_id() + ".json")?, &minecraft_mod)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_id() {
        let test_mod = MinecraftMod::new("test_mod");
        assert_eq!("test_mod", test_mod.get_mod_id())
    }
}
