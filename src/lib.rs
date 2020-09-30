use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct HelloWorld;

#[gdnative::methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.")
    }
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
struct Player;

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
       Player 
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        let mut velocity = Vector2::new(0.0, 0.0);

        let ui_right = GodotString::from_str("ui_right");
        let ui_left = GodotString::from_str("ui_left");
        let ui_up = GodotString::from_str("ui_up");
        let ui_down = GodotString::from_str("ui_down");
        if Input::godot_singleton().is_action_pressed(ui_right) {
            velocity.x += 4_f32;
        } else if Input::godot_singleton().is_action_pressed(ui_left) {
            velocity.x -= 4_f32;
        } else if Input::godot_singleton().is_action_pressed(ui_up) {
            velocity.y -= 4_f32;
        } else if Input::godot_singleton().is_action_pressed(ui_down) {
            velocity.y += 4_f32;
        } else {
            velocity.x = 0_f32;
            velocity.y = 0_f32;
        }
        owner.move_and_collide(velocity, false, false, false);
    }

}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<Player>();
}

godot_init!(init);
