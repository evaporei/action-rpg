use gdnative::prelude::{GodotString, Input, KinematicBody2D, NativeClass, Vector2};

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    velocity: Vector2,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        let ui_right = GodotString::from_str("ui_right");
        let ui_left = GodotString::from_str("ui_left");
        let ui_up = GodotString::from_str("ui_up");
        let ui_down = GodotString::from_str("ui_down");

        let godot_singleton = Input::godot_singleton();

        let right_strength = godot_singleton.get_action_strength(ui_right);
        let left_strength = godot_singleton.get_action_strength(ui_left);
        let down_strength = godot_singleton.get_action_strength(ui_down);
        let up_strength = godot_singleton.get_action_strength(ui_up);

        self.r#move(right_strength, left_strength, down_strength, up_strength);

        owner.move_and_collide(self.velocity, false, false, false);
    }

    fn r#move(
        &mut self,
        right_strength: f64,
        left_strength: f64,
        down_strength: f64,
        up_strength: f64,
    ) {
        let mut input_vector = Vector2::new(0.0, 0.0);

        input_vector.x = (right_strength - left_strength) as f32;
        input_vector.y = (down_strength - up_strength) as f32;

        if input_vector != Vector2::new(0.0, 0.0) {
            self.velocity = input_vector;
        } else {
            self.velocity = Vector2::new(0.0, 0.0);
        }
    }
}
