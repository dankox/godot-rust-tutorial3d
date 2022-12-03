# Squash the creeps (Tutorial 3D Godot/Rust)

This is implementation of the [Godot 3D tutorial game](https://docs.godotengine.org/en/stable/getting_started/first_3d_game/index.html) using GDNative and Rust ([godot-rust](https://github.com/godot-rust/gdnative)) instead of GDScript.

## Setup
Run `setup.sh` to build the native library and create symbolic link in `godot/` directory which is used by Godot.

Start Godot and import the project from `godot/project.godot` file.

***NOTE:** This implementation was build on OSX system, so the other platforms are not tested. If there is some problem with the setup script or gdnlib, update according to your platform.*