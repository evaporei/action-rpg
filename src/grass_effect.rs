use crate::extensions::NodeExt;
use gdnative::api::AnimatedSprite;
use gdnative::prelude::{NativeClass, Node2D};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GrassEffect;

#[gdnative::methods]
impl GrassEffect {
    fn new(_owner: &Node2D) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, owner: &Node2D) {
        let animated_sprite =
            unsafe { owner.get_typed_node::<AnimatedSprite, _>("AnimatedSprite") };

        animated_sprite.play("Animate", false);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_AnimatedSprite_animation_finished(&self, owner: &Node2D) {
        owner.queue_free();
    }
}
