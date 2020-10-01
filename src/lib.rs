use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
struct Player {
    velocity: Vector2,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
       Player { velocity: Vector2::new(0.0, 0.0) }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        let ui_right = GodotString::from_str("ui_right");
        let ui_left = GodotString::from_str("ui_left");
        let ui_up = GodotString::from_str("ui_up");
        let ui_down = GodotString::from_str("ui_down");

        let godot_singleton = Input::godot_singleton();

        let mut input_vector = Vector2::new(0.0, 0.0);
        input_vector.x = (godot_singleton.get_action_strength(ui_right) - godot_singleton.get_action_strength(ui_left)) as f32;
        input_vector.y = (godot_singleton.get_action_strength(ui_down) - godot_singleton.get_action_strength(ui_up)) as f32;

        if input_vector != Vector2::new(0.0, 0.0) {
            self.velocity = input_vector;
        } else {
            self.velocity = Vector2::new(0.0, 0.0);
        }

        owner.move_and_collide(self.velocity, false, false, false);
    }

}

fn init(handle: InitHandle) {
    handle.add_class::<Player>();
}

godot_init!(init);
