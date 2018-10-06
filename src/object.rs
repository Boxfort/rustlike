extern crate tcod;

use object::tcod::console::*;
use object::tcod::Color;
use object::tcod::colors;
use super::map::Map;
use super::player::Player;
use super::components::{
                    RenderComponent,
                    TransformComponent,
                    StatsComponent,
                    AiComponent,
                };

#[derive(Clone)]
pub struct Object{
    renderer: RenderComponent,
    transform: TransformComponent,
    stats: Option<StatsComponent>,
    pub ai: Option<Box<AiComponent>>,
    pub name: String,
    pub blocking: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(
            x: i32,
            y: i32,
            character: char,
            background_color: Option<Color>,
            foreground_color: Option<Color>,
            stats: Option<StatsComponent>,
            ai: Option<Box<AiComponent>>,
            name: String,
            blocking: bool) -> Self {

        let renderer = RenderComponent::new(character,
                                            background_color,
                                            foreground_color);
        let transform = TransformComponent::new(x, y);

        Object {
            renderer,
            transform,
            stats,
            ai,
            name: name,
            blocking: blocking,
            alive: false,
        }
    }

    pub fn stats(&self) -> &Option<StatsComponent> {
        &self.stats
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn position(&self) -> (i32, i32) {
        self.transform.position()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.transform.set_position(x,y);
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map, objects: &Vec<Object>) {
        self.transform.move_by(dx, dy, map, objects);
    }

    pub fn take_turn(&mut self, map: &mut Map, objects: &mut Vec<Object>, player: &mut Player ) {
        if self.ai.is_some() {
            self.ai
                .as_ref()
                .unwrap()
                .clone()
                .take_turn(self, map, objects, player);
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.stats.is_some() {
            self.stats.as_mut().unwrap().hp -= damage;

            if self.stats.as_mut().unwrap().hp <= 0 {
                self.on_death();
            }
        }
    }

    // TODO: This all really needs to be reworked. this should accept
    // some struct that implements 'Attackable' (or something similar)
    // having a statscomponent should be cause enough to fight something.
    // A stats component could be passed in but it wouldnt make sense,
    // and there would be no abstraction allowing for gear and whatnot?
    // Player could be made into an object in future probably if it
    // doesnt require any custom code in it.
    //
    // decide also at what level the damage calculations should be done
    // on the attackers side or on the defenders side. Probably doesnt matter.
    //
    // Decide something for the love of god.
    pub fn attack(&self, player: &mut Player) {
        if self.stats.is_some() {
            // Calculate damage
            let damage = self.stats.as_ref().unwrap().power - player.stats().defence;

            // Apply damage.
            player.take_damage(damage)
        }
    }

    pub fn distance_to(&self, target: (i32, i32)) -> f32 {
        self.transform.distance_to(target)
    }

    pub fn draw(&self, con: &mut Console) {
        self.renderer.draw(con, self.transform.position().0, self.transform.position().1);
    }

    pub fn clear(&self, con: &mut Console) {
        self.renderer.clear(con, self.transform.position().0, self.transform.position().1);
    }

    fn on_death(&mut self) {
        self.alive = false;
        self.blocking = false;
        self.stats = None;
        self.ai = None;
        self.renderer.character = '%';
        self.renderer.background_color = Some(colors::DESATURATED_RED);
    }
}
