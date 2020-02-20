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
            let mut width: i32 = 0;
            for s in tooltip.iter() {
                // Set tooltip width to longest name
                if width < s.len() as i32 {
                    width = s.len() as i32;
                }
            }

            if cursor.x > 40 {
                // Set stuff
            } else {
                // Set other stuff
            }

            let arrow_pos = Point::new(cursor.x - 2, cursor.y);
            let left_x = cursor.x - width;
            let mut y = cursor.y;
            for s in tooltip.iter() {
                // Draw tooltip
                ctx.print_color(
                    left_x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::GREY),
                    s,
                )
            }

            // Print arrow
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"->".to_string(),
            );
        }
    }
}
