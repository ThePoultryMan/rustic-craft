use rustic_craft::MinecraftMod;

fn main() {
    println!("Creating test mod...");

    let test_mod = MinecraftMod::new("rustic-craft-test-mod");

    rustic_craft::create_mod_json_file(test_mod).expect("temporary handling");
}
