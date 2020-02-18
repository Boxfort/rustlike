extern crate specs;
use super::{CombatStats, Name, SufferDamage, WantsToMelee};
use rltk::console;
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut suffer_damage) = data;

        for (_entity, wants_melee, name, stats) in
            (&entities, &mut wants_melee, &names, &combat_stats).join()
        {
            // Dead people can't attack
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                // Don't attack dead people
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    let damage = i32::max(0, stats.power - target_stats.defence);

                    if damage == 0 {
                        console::log(&format!(
                            "{} is unable to hurt {}",
                            &name.name, &target_name.name
                        ));
                    } else {
                        console::log(&format!(
                            "{} hits {} for {} damage",
                            &name.name, &target_name.name, damage
                        ));
                        suffer_damage
                            .insert(wants_melee.target, SufferDamage { amount: damage })
                            .expect("Could not insert damage.");
                    }
                }
            }
        }
        wants_melee.clear();
    }
}
