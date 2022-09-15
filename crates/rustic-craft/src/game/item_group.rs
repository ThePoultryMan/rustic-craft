use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ItemGroup {
    BuildingBlocks,
    Decorations,
    Redstone,
    Transportation,
    Misc,
    Search,
    Food,
    Tools,
    Combat,
    Materials,
}

pub trait TItemGroup {
    fn get_icon(&self) -> String;
}

impl TItemGroup for ItemGroup {
    fn get_icon(&self) -> String {
        match self {
            _ => panic!("You shouldn't be looking for the icon of Minecraft's item groups.")
        }
    }
}