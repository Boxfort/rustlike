extern crate specs;
use super::{CombatStats, GameLog, Name, Potion, WantsToDrinkPotion};
use specs::prelude::*;

pub struct PotionUseSystem {}

impl<'a> System<'a> for PotionUseSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDrinkPotion>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Potion>,
        WriteStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drink,
            names,
            potions,
            mut combat_stats,
        ) = data;

        for (entity, drink, stats) in (&entities, &wants_drink, &mut combat_stats).join() {
            let potion = potions.get(drink.potion);
            if let Some(potion) = potion {
                // Don't heal over max health
                stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                if entity == *player_entity {
                    gamelog.entries.push(format!(
                        "You drink the {}, healing {} hp.",
                        names.get(drink.potion).unwrap().name,
                        potion.heal_amount
                    ));
                }
                entities.delete(drink.potion).expect("Delete failed");
            }
        }

        wants_drink.clear();
    }
}
