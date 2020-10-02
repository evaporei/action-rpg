use gdnative::prelude::{Input, KinematicBody2D, NativeClass, Vector2};

const MAX_SPEED: f32 = 100.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(Default)]
pub struct Player {
    velocity: Vector2,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player::default()
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f64) {
        let godot_singleton = Input::godot_singleton();

        let right_strength = godot_singleton.get_action_strength("ui_right");
        let left_strength = godot_singleton.get_action_strength("ui_left");
        let down_strength = godot_singleton.get_action_strength("ui_down");
        let up_strength = godot_singleton.get_action_strength("ui_up");

        self.r#move(
            right_strength,
            left_strength,
            down_strength,
            up_strength,
            delta,
        );

        owner.move_and_collide(self.velocity, false, false, false);
    }

    fn r#move(
        &mut self,
        right_strength: f64,
        left_strength: f64,
        down_strength: f64,
        up_strength: f64,
        delta: f64,
    ) {
        let mut input_vector = Vector2::new(0.0, 0.0);

        input_vector.x = (right_strength - left_strength) as f32;
        input_vector.y = (down_strength - up_strength) as f32;

        if input_vector != Vector2::new(0.0, 0.0) {
            self.velocity = input_vector;
        } else {
            self.velocity = Vector2::new(0.0, 0.0);
        }

        self.velocity *= (delta as f32) * MAX_SPEED;
    }
}

#[test]
fn test_move_nothing() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, 0.0));
}

#[test]
fn test_move_right() {
    let mut player = Player::default();

    player.r#move(1.0, 0.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(60.000004, 0.0));
}

#[test]
fn test_move_left() {
    let mut player = Player::default();

    player.r#move(0.0, 1.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(-60.000004, 0.0));
}

#[test]
fn test_move_down() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 1.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, 60.000004));
}

#[test]
fn test_move_up() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 0.0, 1.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, -60.000004));
}

#[test]
fn test_move_diagonals() {
    let mut player = Player::default();

    player.r#move(0.0, 1.0, 1.0, 0.0, 0.6);
    player.r#move(0.0, 1.0, 1.0, 0.0, 0.6);
    player.r#move(1.0, 0.0, 0.0, 1.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(60.000002, -60.000002));
}
