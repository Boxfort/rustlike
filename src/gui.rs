extern crate rltk;
extern crate specs;
use super::{CombatStats, Cursor, GameLog, InBackpack, Name, Player, Position, RunState, State};
use rltk::{Console, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

#[derive(PartialEq)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected(Entity),
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    let log = ecs.fetch::<GameLog>();
    let state = *ecs.fetch::<RunState>();

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        // Print Health
        ctx.print_color(
            12,
            43,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );

        // Health Bar
        ctx.draw_bar_horizontal(
            28,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );

        let mut y = 44;
        for s in log.entries.iter().rev() {
            if y < 49 {
                ctx.print(2, y, s);
            }
            y += 1;
        }

        if state == RunState::Examining {
            draw_cursor(ecs, ctx);
        }
    }

    fn draw_cursor(ecs: &World, ctx: &mut Rltk) {
        let cursor = ecs.fetch::<Cursor>();
        let names = ecs.read_storage::<Name>();
        let positions = ecs.read_storage::<Position>();

        let mut tooltip: Vec<String> = Vec::new();

        ctx.print_color(
            cursor.x,
            cursor.y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            &"X".to_string(),
        );

        // Populate tooltip with names of entities under the cursor
        for (name, position) in (&names, &positions).join() {
            if position.x == cursor.x && position.y == cursor.y {
                tooltip.push(name.name.to_string());
            }
        }

        if !tooltip.is_empty() {
            // Draw tooltip
            let mut width_offset: i32 = 0;
            for s in tooltip.iter() {
                // Set tooltip width to longest name
                if width_offset < s.len() as i32 {
                    width_offset = s.len() as i32;
                }
            }

            // Set intial tooltip vars
            let mut arrow = "->".to_string();
            let mut arrow_offset = arrow.len() as i32;
            width_offset += arrow_offset;

            // If we're over halfway then flip the toolip
            if cursor.x > 40 {
                arrow = "<-".to_string();
                width_offset = -(arrow.len() as i32);
                arrow_offset = 0;
            }

            for s in tooltip.iter() {
                // Draw tooltip text
                ctx.print_color(
                    cursor.x - width_offset,
                    cursor.y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::DARK_GRAY),
                    s,
                )
            }

            // Draw tooltip arrow
            ctx.print_color(
                cursor.x - arrow_offset,
                cursor.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::DARK_GRAY),
                &arrow,
            );
        }
    }
}

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> ItemMenuResult {
    show_item_menu("Inventory", gs, ctx)
}

pub fn show_drop_item(gs: &mut State, ctx: &mut Rltk) -> ItemMenuResult {
    show_item_menu("Drop which item?", gs, ctx)
}

fn show_item_menu(title: &str, gs: &mut State, ctx: &mut Rltk) -> ItemMenuResult {
    let player_entity = gs.ecs.fetch::<Entity>();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<InBackpack>();
    let entities = gs.ecs.entities();

    // Get all of the items in the players backpack
    let inventory: Vec<(Entity, &InBackpack, &Name)> = (&entities, &backpack, &names)
        .join()
        .filter(|item| item.1.owner == *player_entity)
        .collect();

    let count = inventory.len() as i32;

    let inventory_y = 25;
    let inventory_x = 15;
    let inventory_width = 32;

    let mut y = inventory_y - (count / 2);

    // Draw the Inventory box
    ctx.draw_box(
        inventory_x,
        y - 2,
        inventory_width,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        inventory_x + 3,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        title,
    );
    ctx.print_color(
        inventory_x + 3,
        y + count + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut usable: Vec<Entity> = Vec::new();
    for (i, (entity, _pack, name)) in inventory.iter().enumerate() {
        // Draw the inventory contents
        ctx.set(
            inventory_x + 2,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            inventory_x + 3,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + i as u8,
        );
        ctx.set(
            inventory_x + 4,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(inventory_x + 6, y, &name.name.to_string());
        usable.push(*entity);

        y += 1;
    }

    match ctx.key {
        None => ItemMenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => ItemMenuResult::Cancel,
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count {
                    return ItemMenuResult::Selected(usable[selection as usize]);
                }
                ItemMenuResult::NoResponse
            }
        },
    }
}
