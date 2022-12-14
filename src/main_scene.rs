use gdnative::api::PathFollow;
use gdnative::prelude::*;
use rand::Rng;

use crate::mob;
use crate::utils;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Main {
    #[property]
    mob_scene: Ref<PackedScene>,
    retry: Option<Ref<ColorRect>>,
}

#[methods]
impl Main {
    fn new(_owner: &Node) -> Self {
        Main {
            mob_scene: PackedScene::new().into_shared(),
            retry: None,
        }
    }

    #[method]
    fn _ready(&mut self, #[base] owner: &Node) {
        unsafe {
            self.retry = Some(
                owner
                    .get_node_as::<ColorRect>("UserInterface/Retry")
                    .unwrap()
                    .claim(),
            );
            self.retry.unwrap().assume_safe().hide();
        }
    }

    #[method]
    fn _unhandled_input(&self, #[base] owner: &Node, event: Ref<InputEvent>) {
        unsafe {
            if event.assume_safe().is_action_pressed("ui_accept", false, false)
                && self.retry.unwrap().assume_safe().is_visible()
            {
                owner
                    .get_tree()
                    .unwrap()
                    .assume_safe()
                    .reload_current_scene()
                    .unwrap_or_else(|e| godot_print!("failed to reload scene! {}", e))
            }
        }
    }

    #[method]
    fn on_mobtimer_timeout(&mut self, #[base] owner: &Node) {
        let mob_scene: Ref<KinematicBody, _> = utils::instance_scene(&self.mob_scene);
        let mob_spawn_location = unsafe {
            owner
                .get_node_as::<PathFollow>("SpawnPath/SpawnLocation")
                .unwrap()
        };
        let mut rng = rand::thread_rng();
        let offset: f64 = rng.gen();
        mob_spawn_location.set_unit_offset(offset);

        // handle player position even if it's not there (a.k.a. is dead!)
        let player_position: Vector3;
        let player = unsafe { owner.get_node_as::<KinematicBody>("Player") };
        match player {
            Some(p) => player_position = p.translation(),
            None => {
                player_position = Vector3 {
                    x: rng.gen_range(-14.0..=14.0),
                    y: 0.0,
                    z: rng.gen_range(-14.0..=16.0),
                }
            }
        }

        // add mob to scene and initialize it
        let mob_scene = unsafe { mob_scene.into_shared().assume_safe() };
        owner.add_child(mob_scene, false);
        let mob = mob_scene.cast_instance::<mob::Mob>().unwrap();
        mob.map_mut(|m, node| {
            m.initialize(&node, mob_spawn_location.translation(), player_position);
            let sl = unsafe {
                owner
                    .get_node_as::<Label>("UserInterface/ScoreLabel")
                    .unwrap()
            };
            node.connect(
                "squashed",
                sl,
                "on_mob_squashed",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
        })
        .ok()
        .unwrap_or_else(|| godot_print!("unable to get mob"));
    }

    #[method]
    fn on_player_hit(&mut self, #[base] owner: &Node) {
        unsafe {
            let timer = owner.get_node_as::<Timer>("MobTimer").unwrap();
            timer.stop();
            self.retry.unwrap().assume_safe().show();
        };
    }
}
