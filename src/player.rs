use gdnative::{api::CollisionObject, prelude::*};

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    #[property(default = 14.0)]
    speed: f32,
    #[property(default = 75.0)]
    fall_acceleration: f32,
    #[property(default = 20.0)]
    jump_impulse: f32,
    #[property(default = 16.0)]
    bounce_impulse: f32,

    velocity: Vector3,
    // TODO: keep references for objects which are used?
    // pivot: TRef<'a, Spatial>,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody) -> Self {
        Player {
            speed: 14.0,
            fall_acceleration: 75.0,
            jump_impulse: 20.0,
            bounce_impulse: 16.0,
            velocity: Vector3::ZERO,
        }
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody, delta: f32) {
        let mut direction = Vector3::ZERO;
        let input = Input::godot_singleton();

        // check for input
        if input.is_action_pressed("move_right", false) {
            direction.x += 1.0
        }
        if input.is_action_pressed("move_left", false) {
            direction.x -= 1.0
        }
        if input.is_action_pressed("move_forward", false) {
            direction.z -= 1.0
        }
        if input.is_action_pressed("move_back", false) {
            direction.z += 1.0
        }

        // rotate towards direction
        if direction != Vector3::ZERO {
            direction = direction.normalized();
            let pivot = unsafe { owner.get_node_as::<Spatial>("Pivot").unwrap() };
            pivot.look_at(owner.translation() + direction, Vector3::UP)
        }

        // ground velocity
        self.velocity.x = direction.x * self.speed;
        self.velocity.z = direction.z * self.speed;
        // gravitation velocity
        self.velocity.y -= self.fall_acceleration * delta;
        // add jump to velocity
        if owner.is_on_floor() && input.is_action_just_pressed("jump", false) {
            self.velocity.y += self.jump_impulse
        }
        // move the player (last 4 args are defaults)
        self.velocity = owner.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.785398, true);

    }
}
