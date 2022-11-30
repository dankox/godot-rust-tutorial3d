use gdnative::prelude::*;

mod mob;
mod player;
mod main_scene;
mod score_label;
mod utils;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::Player>();
    handle.add_class::<mob::Mob>();
    handle.add_class::<main_scene::Main>();
    handle.add_class::<score_label::Score>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
