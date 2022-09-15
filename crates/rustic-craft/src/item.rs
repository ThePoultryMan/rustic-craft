use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Item {
    identifier: String,
    item_settings: ItemSettings,
}

#[derive(Deserialize, Serialize)]
pub struct ItemSettings {
    item_group: ItemGroup,
}

#[derive(Deserialize, Serialize)]
pub enum ItemGroup {
    Combat,
    Decorations,
    Search,
}

trait TItemGroup {
    fn get_icon(&self) -> String;
}

impl Item {
    pub fn new(identifier: &str, item_settings: ItemSettings) -> Self {
        Self {
            identifier: identifier.to_owned(),
            item_settings,
        }
    }
}

impl ItemSettings {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn group(&mut self, item_group: ItemGroup) -> &mut Self {
        self.item_group = item_group;
        self
    }
}

impl TItemGroup for ItemGroup {
    fn get_icon(&self) -> String {
        match self {
            _ => panic!("You shouldn't be looking for the icon of Minecraft's item groups.")
        }
    }
}

impl Default for ItemSettings {
    fn default() -> Self {
        Self { item_group: ItemGroup::Search }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_item() {
        let cargo_test = Item::new("cargo_test", ItemSettings::new());
        assert_eq!(cargo_test.identifier, "cargo_test");
    }

    #[test]
    #[should_panic]
    fn minecrafts_item_group_icon() {
        ItemGroup::Combat.get_icon();
    }
}