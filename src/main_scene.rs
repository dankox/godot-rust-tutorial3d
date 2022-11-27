use gdnative::api::PathFollow;
use gdnative::prelude::*;
use rand::Rng;

use crate::mob;
use crate::utils;
// use crate::player;
// use std::f64::consts::PI;

// static RAND : TRef<RandomNumberGenerator> = RandomNumberGenerator::new();

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main {
    #[property]
    mob_scene: Ref<PackedScene>,
    // velocity: Vector3,
}

#[methods]
impl Main {
    fn new(_owner: &Node) -> Self {
        Main {
            mob_scene: PackedScene::new().into_shared(),
            // velocity: Vector3::ZERO,
        }
    }

    // #[method]
    // fn _physics_process(&mut self, #[base] owner: &Node, _delta: f32) {
    //     // move the player (last 4 args are defaults)
    //     // owner.move_and_slide(self.velocity, Vector3::UP, false, 4, 0.785398, true);
    // }

    #[method]
    fn on_mobtimer_timeout(&mut self, #[base] owner: &Node) {
        // let mob_scene: Ref<KinematicBody, _> = instance_scene(&self.mob_scene);
        let mob_scene: Ref<KinematicBody, _> = utils::instance_scene(&self.mob_scene);
        // let mob: mob::Mob = unsafe {
        //     mob.try_cast::<mob::Mob>().expect("mob_scene should be Mob struct");
        // };
        let mob_spawn_location = unsafe {
            owner
                .get_node_as::<PathFollow>("SpawnPath/SpawnLocation")
                .unwrap()
        };
        // let mob_spawn_location = owner.get_node("path").expect("bla");
                // .get_node_as::<PathFollow>("SpawnPath/SpawnLocation")
                // .unwrap();
        let mut rng = rand::thread_rng();
        let offset: f64 = rng.gen();
        mob_spawn_location.set_offset(offset);

        let player: TRef<KinematicBody> = unsafe { owner.get_node_as::<KinematicBody>("Player").unwrap() };
        // let bla = Option;
        // let player: TRef<KinematicBody> = unsafe { owner.get_node_as::<KinematicBody>("Player").ok().unwrap_or_else(||godot_print!("wtf??")) };
        let player_position = player.translation();

        let mob_scene = unsafe {mob_scene.into_shared().assume_safe()};
        owner.add_child(mob_scene, false);

        let mob = mob_scene.cast_instance::<mob::Mob>().unwrap();

        // let b = mob::Mob::new(mob_scene.as_ref());
        // b.initialize(owner, start_position, player_position);

        // let mut mmob: mob::Mob = mob.clone();
        // let mob = mob.claim();
        // let b: &mob::Mob = mob.script().try_into().as_ref().ok().unwrap();
        mob.map_mut(|x: &mut mob::Mob, mob_owner| x.initialize(&mob_owner, mob_spawn_location.translation(), player_position))
        .ok()
        .unwrap_or_else(|| godot_print!("unable to get mob"));
        // mob.map(|x, mob_owner| x.initialize(&mob_owner, mob_spawn_location.translation(), player_position))
        // .ok()
        // .unwrap_or_else(|| godot_print!("unable to get mob"));
        
        // initialize(mob_spawn_location.translation(), player_position);
    }
}

// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     let mut i:i32 = 1;
//     let ref_i:&mut i32 = &mut i;
//     *ref_i = 2;
//     i = 3;
//     if i == 3 {
//         println!("3")
//     }
//     // let ss: str;
//     let s: &str = "test";

//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// fn test_longest() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
//     // println!("The longest string is {}", result);
// }
