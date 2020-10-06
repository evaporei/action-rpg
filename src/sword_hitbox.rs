use gdnative::api::Area2D;
use gdnative::prelude::{NativeClass, Vector2};

#[derive(NativeClass)]
#[inherit(Area2D)]
#[derive(Default)]
pub struct SwordHitbox {
    pub(crate) knockback_vector: Vector2,
}

#[gdnative::methods]
impl SwordHitbox {
    fn new(_owner: &Area2D) -> Self {
        Self::default()
    }
}
