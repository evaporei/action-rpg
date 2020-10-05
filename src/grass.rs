use crate::load_scene;
use gdnative::prelude::{Input, NativeClass, Node2D, TRef};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass;

#[gdnative::methods]
impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Self
    }

    #[export]
    fn _process(&self, owner: &Node2D, _delta: f32) {
        let input_singleton = Input::godot_singleton();

        if input_singleton.is_action_just_pressed("attack") {
            let grass_effect_scene = load_scene("res://scenes/GrassEffect.tscn").unwrap();

            let grass_effect_node =
                unsafe { grass_effect_scene.instance(0).unwrap().assume_safe() };
            let grass_effect: TRef<Node2D> = grass_effect_node.cast().unwrap();

            grass_effect.set_global_position(owner.global_position());

            let grass_parent = unsafe { owner.get_parent().unwrap().assume_safe() };

            grass_parent.add_child(grass_effect, false);

            owner.queue_free();
        }
    }
}
