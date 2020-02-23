extern crate rltk;
extern crate specs;
use super::{CombatStats, Cursor, GameLog, Name, Player, Position, RunState};
use rltk::{Console, Point, Rltk, RGB};
use specs::prelude::*;

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

            // Set intial tooltip
            let mut arrow = "->".to_string();
            let mut arrow_offset = arrow.len() as i32;
            width_offset += arrow_offset;

            // If we're over halfway then flip the toolip
            if cursor.x > 40 {
                arrow = "<-".to_string();
                width_offset = -(arrow.len() as i32);
                arrow_offset = 0;
            }

            let arrow_pos = Point::new(cursor.x, cursor.y);
            let mut y = cursor.y;
            for (idx, s) in tooltip.iter().enumerate() {
                // Draw tooltip text
                ctx.print_color(
                    cursor.x - width_offset,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::DARK_GRAY),
                    s,
                )
            }

            // Draw tooltip arrow
            ctx.print_color(
                arrow_pos.x - arrow_offset,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::DARK_GRAY),
                &arrow,
            );
        }
    }
}
