use gdnative::prelude::*;
// use gdnative::api::RandomNumberGenerator;

mod mob;
mod player;
mod main_scene;
mod utils;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // let rand = RandomNumberGenerator::new();
    handle.add_class::<player::Player>();
    handle.add_class::<mob::Mob>();
    handle.add_class::<main_scene::Main>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
