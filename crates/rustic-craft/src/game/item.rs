use serde::{Deserialize, Serialize};

use super::item_group::ItemGroup;

#[derive(Deserialize, Serialize)]
pub struct Item {
    identifier: String,
    item_settings: ItemSettings,
}

#[derive(Deserialize, Serialize)]
pub struct ItemSettings {
    item_group: ItemGroup,
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

impl Default for ItemSettings {
    fn default() -> Self {
        Self { item_group: ItemGroup::Search }
    }
}

#[cfg(test)]
mod test {
    use crate::game::item_group::TItemGroup;

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