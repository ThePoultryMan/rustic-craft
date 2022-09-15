use rustic_craft::{MinecraftMod, game::item::{Item, ItemSettings}};

fn main() {
    println!("Creating test mod...");

    let mut test_mod = MinecraftMod::new("rustic_craft_example");

    let test_item = Item::new("test_item", ItemSettings::new());

    test_mod.register_item(test_item);

    rustic_craft::create_mod_json_file(test_mod).expect("temporary error handling");
}
