extern crate specs;
use super::{
    CombatStats, GameLog, InBackpack, Name, Position, Potion, WantsToDrinkPotion, WantsToDropItem,
};
use specs::prelude::*;

pub struct PotionUseSystem {}
pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToDropItem>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut gamelog,
            entities,
            mut wants_drop,
            names,
            mut positions,
            mut backpack,
        ) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut drop_position: Position = Position { x: 0, y: 0 };
            {
                let pos = positions.get(entity).unwrap();
                drop_position.x = pos.x;
                drop_position.y = pos.y;
            }
            positions
                .insert(
                    to_drop.item,
                    Position {
                        x: drop_position.x,
                        y: drop_position.y,
                    },
                )
                .expect("Unable to insert position");
            backpack.remove(to_drop.item);

            if entity == *player_entity {
                gamelog.entries.push(format!(
                    "You drop the {}.",
                    names.get(to_drop.item).unwrap().name
                ))
            }
        }

        wants_drop.clear();
    }
}

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
