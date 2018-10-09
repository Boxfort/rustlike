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
    message_log: MessageLog,
}

struct ProgressBar {
    x: i32,
    y: i32,
    console_x: i32,
    console_y: i32,
    console_width: i32,
    console_height: i32,
    bar_width: i32,
    value: i32,
    maximum: i32,
    full_color: Color,
    empty_color: Color,
    name: String,
    console: Box<Console>,
}

struct Text {
    x: i32,
    y: i32,
    text: String,
    color: Color,
    console: Box<Console>,
}

struct MessageLog {
    x: i32,
    y: i32,
    messages: Vec<(String, Color)>,
    console_x: i32,
    console_y: i32,
    console_width: i32,
    console_height: i32,
    console: Box<Console>,
}

trait GuiElement {
    fn draw(&mut self, root: &mut Root);
}

impl Gui {
    pub fn new(screen_width: i32, screen_height: i32, player: &Player) -> Self {
        let mut elements: Vec<Box<GuiElement>> = vec![];

        let hp = ProgressBar::new(1,
                                  1,
                                  0,
                                  screen_height - 7,
                                  20,
                                  screen_width,
                                  7,
                                  player.stats().hp,
                                  player.stats().max_hp,
                                  colors::LIGHT_RED,
                                  colors::DARKER_RED,
                                  "HP".to_string());

        let message_log = MessageLog::new(0,
                                          0,
                                          vec![],
                                          22,
                                          screen_height - 6,
                                          screen_width - 22,
                                          6);

        Gui {
            health_bar: hp,
            message_log: message_log,
        }
    }

    pub fn draw(&mut self, root: &mut Root) {
        self.health_bar.draw(root);
        self.message_log.draw(root);
    }

    pub fn update(&mut self, player: &Player, messages: &Vec<(String, Color)>) {
        self.health_bar.value = player.stats().hp;
        self.message_log.messages = messages.clone();
    }
}

impl ProgressBar {
    pub fn new(x: i32,
               y: i32,
               console_x: i32,
               console_y: i32,
               bar_width: i32,
               console_width: i32,
               console_height: i32,
               value: i32,
               maximum: i32,
               full_color: Color,
               empty_color: Color,
               name: String)
        -> Self
    {
        let console = Box::new(Offscreen::new(console_width, console_height));

        ProgressBar {
            x,
            y,
            console_x,
            console_y,
            console_width,
            console_height,
            bar_width,
            value,
            maximum,
            full_color,
            empty_color,
            name,
            console,
        }
    }
}

impl MessageLog {
    pub fn new(x: i32,
               y: i32,
               messages: Vec<(String, Color)>,
               console_x: i32,
               console_y: i32,
               console_width: i32,
               console_height: i32)
        -> Self
    {
        let console = Box::new(Offscreen::new(console_width, console_height));

        MessageLog {
            x,
            y,
            messages,
            console_x,
            console_y,
            console_width,
            console_height,
            console,
        }
    }
}

impl GuiElement for MessageLog {
    fn draw(&mut self, root: &mut Root) {
        self.console.clear();
        let mut y = self.console_height - 1;
        for &(ref msg, color) in self.messages.iter().rev() {
            let msg_height = self.console
                                 .get_height_rect(self.console_x,
                                                  y,
                                                  self.console_width,
                                                  0,
                                                  msg);
            y -= msg_height;
            if y < 0 {
                break;
            }

            self.console.set_default_foreground(color);
            self.console.print_rect(self.x, y, self.console_width, 0, msg);

            blit(&mut self.console,
                 (0,0),
                 (self.console_width, self.console_height),
                 root,
                 (self.console_x, self.console_y),
                 1.0,
                 1.0);
        }
    }
}

impl GuiElement for ProgressBar {
    fn draw(&mut self, root: &mut Root) {
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
                         format!("{}: {}/{} ", self.name, self.value, self.maximum));

        blit(&mut self.console,
             (0,0),
             (self.console_width, self.console_height),
             root,
             (self.console_x, self.console_y),
             1.0,
             1.0);
    }
}
