extern crate specs;
use super::{CombatStats, GameLog, Name, Player, SufferDamage};
use rltk::console;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount;
        }

        damage.clear();
    }
}

impl DamageSystem {
    pub fn delete_the_dead(ecs: &mut World) {
        let mut dead: Vec<Entity> = Vec::new();
        {
            let mut log = ecs.fetch_mut::<GameLog>();
            let players = ecs.read_storage::<Player>();
            let combat_stats = ecs.read_storage::<CombatStats>();
            let names = ecs.read_storage::<Name>();
            let entities = ecs.entities();

            for (entity, stats) in (&entities, &combat_stats).join() {
                if stats.hp < 1 {
                    let player = players.get(entity);
                    match player {
                        None => {
                            let victim_name = names.get(entity);
                            if let Some(victim_name) = victim_name {
                                log.entries.push(format!("{} is dead", &victim_name.name));
                            }
                            dead.push(entity)
                        }
                        Some(_) => console::log("You are dead"),
                    }
                }
            }
        }

        for victim in dead {
            ecs.delete_entity(victim).expect("Unable to delete entity.")
        }
    }
}
