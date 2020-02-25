use super::{
    CombatStats, Cursor, GameLog, Item, Map, Player, Point, Position, RunState, State, Viewshed,
    WantsToMelee, WantsToPickupItem,
};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut ppos = ecs.write_resource::<Point>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        // Make sure the move is in bounds
        if !map.is_in_bounds(pos.x + delta_x, pos.y + delta_y) {
            return;
        }

        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            if combat_stats.get(*potential_target).is_some() {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Adding melee target failed.");
                return;
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn try_move_cursor(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut cursor = ecs.fetch_mut::<Cursor>();
    let map = ecs.fetch::<Map>();

    if !map.is_in_bounds(cursor.x + delta_x, cursor.y + delta_y) {
        return;
    }

    cursor.x += delta_x;
    cursor.y += delta_y;
}

/// Handle players input, carries out appropriate actions, and returns
/// the resulting state.
///
/// This is essentially a state machine, and so in future may need to be
/// re-implemented as such.
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let state = *gs.ecs.fetch::<RunState>();

    match ctx.key {
        None => return state, // Nothing to do.
        Some(key) => match key {
            // Cardinal Directions
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                handle_movement(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                handle_movement(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                handle_movement(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                handle_movement(0, 1, &mut gs.ecs)
            }
            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => handle_movement(1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => handle_movement(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => handle_movement(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => handle_movement(-1, 1, &mut gs.ecs),
            // Handle Examining
            VirtualKeyCode::X => return handle_examine(&mut gs.ecs),
            // Handle Pickup
            VirtualKeyCode::G => return get_item(&mut gs.ecs),
            // Handle Inventory
            VirtualKeyCode::I => return RunState::ShowInventory,
            // Handle Dropping
            VirtualKeyCode::D => return RunState::ShowDropItem,
            // No key is being pressed so we're still waiting for input
            _ => return state,
        },
    }

    if state == RunState::Examining {
        state
    } else {
        RunState::MonsterTurn
    }
}

/// Handles switching between examining state
fn handle_examine(ecs: &mut World) -> RunState {
    let state = *ecs.fetch::<RunState>();

    if state == RunState::AwaitingInput {
        // Fetch the needed information
        let mut cursor = ecs.fetch_mut::<Cursor>();
        let player_pos = ecs.fetch::<Point>();

        // Put the cursor on top of the player
        cursor.x = player_pos.x;
        cursor.y = player_pos.y;

        RunState::Examining
    } else {
        RunState::AwaitingInput
    }
}

fn handle_movement(x: i32, y: i32, ecs: &mut World) {
    let state = *ecs.fetch::<RunState>();

    match state {
        RunState::Examining => try_move_cursor(x, y, ecs),
        _ => try_move_player(x, y, ecs),
    }
}

fn get_item(ecs: &mut World) -> RunState {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
            break;
        }
    }

    match target_item {
        None => {
            gamelog
                .entries
                .push("There is nothing here to pick up.".to_string());
            RunState::AwaitingInput
        }
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
            RunState::MonsterTurn
        }
    }
}
