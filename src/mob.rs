use gdnative::prelude::*;
use rand::*;
use std::f64::consts::PI;
use std::ops::Mul;

// static RAND : TRef<RandomNumberGenerator> = RandomNumberGenerator::new();

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Mob {
    #[property(default = 10.0)]
    min_speed: f32,
    #[property(default = 18.0)]
    max_speed: f32,

    velocity: Vector3,
}

#[methods]
impl Mob {
    pub fn new(_owner: &KinematicBody) -> Self {
        Mob {
            min_speed: 10.0,
            max_speed: 18.0,
            velocity: Vector3::ZERO,
        }
    }

    #[method]
    pub fn initialize(
        &mut self,
        #[base] owner: &KinematicBody,
        start_position: Vector3,
        player_position: Vector3,
    ) {
        // prepare random generator
        let mut rng = rand::thread_rng();
        // set rotation
        owner.look_at_from_position(start_position, player_position, Vector3::UP);
        let random_angle: f64 = rng.gen_range(-PI / 4.0..PI / 4.0);
        owner.rotate_y(random_angle);

        // calculate and set speed
        let random_speed = rng.gen_range(self.min_speed..=self.max_speed);
        self.velocity = Vector3::FORWARD.mul(random_speed);
        self.velocity = self.velocity.rotated(Vector3::UP, owner.rotation().y);
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody, _delta: f32) {
        // move the player (last 4 args are defaults)
        owner.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.785398, true);
    }

    #[method]
    fn on_screen_exited(&mut self, #[base] owner: &KinematicBody) {
        owner.queue_free()
    }
}
