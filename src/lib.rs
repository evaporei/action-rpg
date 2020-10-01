use gdnative::prelude::{godot_init, InitHandle};

mod player;

use player::Player;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
