use gdnative::prelude::{godot_init, InitHandle, PackedScene, Ref, ResourceLoader, ThreadLocal};

mod bat;
mod extensions;
mod grass;
mod grass_effect;
mod player;
mod sword_hitbox;

use bat::Bat;
use grass::Grass;
use grass_effect::GrassEffect;
use player::Player;
use sword_hitbox::SwordHitbox;

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
    handle.add_class::<Bat>();
    handle.add_class::<SwordHitbox>();
    handle.add_class::<Grass>();
    handle.add_class::<GrassEffect>();
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

godot_init!(init);
