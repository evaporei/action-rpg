use gdnative::prelude::{godot_init, InitHandle, PackedScene, Ref, ResourceLoader, ThreadLocal};

mod bat;
mod effect;
mod extensions;
mod grass;
mod player;
mod stats;
mod sword_hitbox;

use bat::Bat;
use effect::Effect;
use grass::Grass;
use player::Player;
use stats::Stats;
use sword_hitbox::SwordHitbox;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Bat>();
    handle.add_class::<Stats>();
    handle.add_class::<SwordHitbox>();
    handle.add_class::<Grass>();
    handle.add_class::<Effect>();
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    // this already caches the resource if it has already been loaded.
    // This is defined by the third argument
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

godot_init!(init);
