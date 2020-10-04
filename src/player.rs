use crate::extensions::NodeExt;
use gdnative::api::AnimationPlayer;
use gdnative::prelude::{Input, KinematicBody2D, NativeClass, Vector2, Vector2Godot};

const ACCELERATION: f32 = 500.0;
const MAX_SPEED: f32 = 80.0;
const FRICTION: f32 = 500.0;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[derive(Default)]
pub struct Player {
    velocity: Vector2,
}

#[gdnative::methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            velocity: Vector2::zero(),
        }
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        let animation_player =
            unsafe { owner.get_typed_node::<AnimationPlayer, _>("AnimationPlayer") };

        let input_vector = self.get_movement_input();

        self.animate(&animation_player, input_vector);

        self.r#move(input_vector, delta);

        self.velocity =
            owner.move_and_slide(self.velocity, Vector2::zero(), false, 4, 0.785398, true);
    }

    fn get_movement_input(&self) -> Vector2 {
        let godot_singleton = Input::godot_singleton();

        let right_strength = godot_singleton.get_action_strength("ui_right");
        let left_strength = godot_singleton.get_action_strength("ui_left");
        let down_strength = godot_singleton.get_action_strength("ui_down");
        let up_strength = godot_singleton.get_action_strength("ui_up");

        let mut input_vector = Vector2::zero();

        input_vector.x = (right_strength - left_strength) as f32;
        input_vector.y = (down_strength - up_strength) as f32;

        input_vector.try_normalize().unwrap_or(input_vector)
    }

    fn animate(&self, animation_player: &AnimationPlayer, input_vector: Vector2) {
        if input_vector != Vector2::zero() {
            if input_vector.x > 0.0 {
                animation_player.play("RunRight", -1.0, 1.0, false);
            } else {
                animation_player.play("RunLeft", -1.0, 1.0, false);
            }
        } else {
            animation_player.play("IdleRight", -1.0, 1.0, false);
        }
    }

    fn r#move(&mut self, input_vector: Vector2, delta: f32) {
        if input_vector != Vector2::zero() {
            self.velocity = self
                .velocity
                .move_towards(input_vector * MAX_SPEED, ACCELERATION * delta);
        } else {
            self.velocity = self
                .velocity
                .move_towards(Vector2::zero(), FRICTION * delta);
        }
    }
}

#[test]
fn test_move_nothing() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::zero());
}

#[test]
fn test_move_right() {
    let mut player = Player::default();

    player.r#move(1.0, 0.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(1.0 * MAX_SPEED, 0.0));
}

#[test]
fn test_move_left() {
    let mut player = Player::default();

    player.r#move(0.0, 1.0, 0.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(-1.0 * MAX_SPEED, 0.0));
}

#[test]
fn test_move_down() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 1.0, 0.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, 1.0 * MAX_SPEED));
}

#[test]
fn test_move_up() {
    let mut player = Player::default();

    player.r#move(0.0, 0.0, 0.0, 1.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(0.0, -1.0 * MAX_SPEED));
}

#[ignore]
#[test]
fn test_move_diagonals() {
    let mut player = Player::default();

    player.r#move(0.0, 1.0, 1.0, 0.0, 0.6);
    player.r#move(0.0, 1.0, 1.0, 0.0, 0.6);
    player.r#move(1.0, 0.0, 0.0, 1.0, 0.6);

    assert_eq!(player.velocity, Vector2::new(-4.242641, 4.242641));
}
