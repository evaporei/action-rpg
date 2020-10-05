use crate::load_scene;
use gdnative::api::Area2D;
use gdnative::prelude::{NativeClass, Node2D, Ref, TRef};

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Grass;

#[gdnative::methods]
impl Grass {
    fn new(_owner: &Node2D) -> Self {
        Self
    }

    fn create_grass_effect(&self, owner: &Node2D) {
        let grass_effect_scene = load_scene("res://scenes/GrassEffect.tscn").unwrap();

        let grass_effect_node = unsafe { grass_effect_scene.instance(0).unwrap().assume_safe() };
        let grass_effect: TRef<Node2D> = grass_effect_node.cast().unwrap();

        grass_effect.set_global_position(owner.global_position());

        let grass_parent = unsafe { owner.get_parent().unwrap().assume_safe() };

        grass_parent.add_child(grass_effect, false);
    }

    #[export]
    #[allow(non_snake_case)]
    fn _on_Hurtbox_area_entered(&self, owner: &Node2D, _area: Ref<Area2D>) {
        self.create_grass_effect(owner);

        owner.queue_free();
    }
}
