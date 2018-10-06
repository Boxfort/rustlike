extern crate tcod;

use gui::tcod::Color;
use gui::tcod::console::Root;
use gui::tcod::console::Console;
use gui::tcod::console::Offscreen;
use gui::tcod::console::blit;
use gui::tcod::BackgroundFlag;
use gui::tcod::TextAlignment;
use gui::tcod::colors;
use super::player::Player;

pub struct Gui {
    health_bar: ProgressBar,
}

struct ProgressBar {
    x: i32,
    y: i32,
    bar_width: i32,
    value: i32,
    maximum: i32,
    full_color: Color,
    empty_color: Color,
    console: Box<Console>,
}

struct Text {
    x: i32,
    y: i32,
    text: String,
    color: Color,
    console: Box<Console>,
}

trait GuiElement {
    fn draw(&mut self, root: &mut Root, screen_width: i32, screen_height: i32);
    fn clear(&mut self);
}

impl Gui {
    pub fn new(screen_width: i32, screen_height: i32, player: &Player) -> Self {
        let mut elements: Vec<Box<GuiElement>> = vec![];

        let hp = ProgressBar::new(1,
                                  1,
                                  20,
                                  screen_width,
                                  player.stats().hp,
                                  player.stats().max_hp,
                                  colors::LIGHT_RED,
                                  colors::DARKER_RED);


        Gui {
            health_bar: hp,
        }
    }


    pub fn draw(&mut self, root: &mut Root, screen_width: i32, screen_height: i32) {
        self.health_bar.draw(root, screen_width, screen_height);
    }

    pub fn update(&mut self, player: &Player) {
        self.health_bar.value = player.stats().hp;
    }

    pub fn clear(&mut self) {
        self.health_bar.clear();
    }
}

impl ProgressBar {
    pub fn new(x: i32,
               y: i32,
               bar_width: i32,
               console_width: i32,
               value: i32,
               maximum: i32,
               full_color: Color,
               empty_color: Color)
        -> Self
    {
        let console = Box::new(Offscreen::new(console_width, 7));

        ProgressBar {
            x,
            y,
            bar_width,
            value,
            maximum,
            full_color,
            empty_color,
            console,
        }
    }
}

impl GuiElement for ProgressBar {
    fn draw(&mut self, root: &mut Root, screen_width: i32, screen_height: i32) {
        self.console.set_default_background(colors::BLACK);
        self.console.clear();

        // render a bar (HP, experience, etc). First calculate the width of the bar
        let bar_width = (self.value as f32 / self.maximum as f32 * self.bar_width as f32) as i32;

        // render the background first
        self.console.set_default_background(self.empty_color);
        self.console.rect(self.x,
                          self.y,
                          self.bar_width,
                          1,
                          false,
                          BackgroundFlag::Screen);

        // now render the bar on top
        self.console.set_default_background(self.full_color);
        if bar_width > 0 {
            self.console.rect(self.x,
                              self.y,
                              bar_width,
                              1,
                              false,
                              BackgroundFlag::Screen);
        }

        // show the player's stats
        self.console.set_default_foreground(colors::WHITE);
        self.console.print_ex(self.x, self.y + 2, BackgroundFlag::None, TextAlignment::Left,
                         format!("HP: {}/{} ", self.value, self.maximum));

        blit(&mut self.console, (0,0), (screen_width, 7), root, (0, screen_height - 7), 1.0, 1.0);
        println!("DRAWEN")
    }

    fn clear(&mut self) {
        self.console.clear();
    }
}
