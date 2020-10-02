use gdnative::prelude::{Input, KinematicBody2D, NativeClass, Vector2, Vector2Godot};

const ACCELERATION: f64 = 10.0;
const MAX_SPEED: f64 = 100.0;
const FRICTION: f64 = 10.0;

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
            input_vector = input_vector.normalize();

            self.velocity += input_vector * ACCELERATION as f32 * delta as f32;
            clamped(&mut self.velocity, (MAX_SPEED * delta) as f32);
        } else {
            self.velocity = self
                .velocity
                .move_towards(Vector2::new(0.0, 0.0), (FRICTION * delta) as f32);
        }
    }
}

fn clamped(v: &mut Vector2, p_len: f32) {
    if p_len < 1.0 {
        *v /= 1.0;
        *v *= p_len;
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

    assert_eq!(player.velocity, Vector2::new(15.000001, 0.0));
}

#[test]
fn test_move_left() {
    let mut player = Player::default();

    player.r#move(0.0, 1.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(-1800.0002, 0.0));
}

#[test]
fn test_move_down() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 1.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, 15.000001));
}

#[test]
fn test_move_up() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 0.0, 1.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, -1800.0002));
}

#[test]
fn test_move_diagonals() {
    let mut player = Player::default();

    player.r#move(0.0, 1.0, 1.0, 0.0, 0.6);
    player.r#move(0.0, 1.0, 1.0, 0.0, 0.6);
    player.r#move(1.0, 0.0, 0.0, 1.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(-18_479_672.0, 10.606602));
}
