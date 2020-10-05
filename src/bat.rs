use gdnative::api::Area2D;
use gdnative::prelude::{KinematicBody2D, NativeClass, Ref};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Bat;

#[gdnative::methods]
impl Bat {
    fn new(_owner: &KinematicBody2D) -> Self {
        Self
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_Hurtbox_area_entered(&self, owner: &KinematicBody2D, _area: Ref<Area2D>) {
        owner.queue_free()
    }
}
