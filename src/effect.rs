use gdnative::api::AnimatedSprite;
use gdnative::prelude::{NativeClass, TRef, VariantArray};

#[derive(NativeClass)]
#[inherit(AnimatedSprite)]
pub struct Effect;

#[gdnative::methods]
impl Effect {
    fn new(_owner: &AnimatedSprite) -> Self {
        Self
    }

    #[export]
    fn _ready(&self, owner: TRef<AnimatedSprite>) {
        owner
            .connect(
                "animation_finished",
                owner,
                "_on_animation_finished",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        owner.play("Animate", false);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_animation_finished(&self, owner: &AnimatedSprite) {
        owner.queue_free();
    }
}
