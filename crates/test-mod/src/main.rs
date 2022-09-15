use rustic_craft::{MinecraftMod, item::Item};

fn main() {
    println!("Creating test mod...");

    let mut test_mod = MinecraftMod::new("rustic-craft-example-mod");

    let test_item = Item::new("test_item");

    test_mod.register_item(test_item);

    rustic_craft::create_mod_json_file(test_mod).expect("temporary error handling");
}
