mod config;
mod game;

use std::{
    io::{stdin, stdout, Write},
    process::{self, exit},
};

use config::Config;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
};
use game::{Game, VictoryState};

fn main() {
    println!("minesweeper time");
    let mut controller = Controller::new(Config::new());
    controller.start();
}

struct Controller {
    game: Game,
    debug: bool,
}

enum Action {
    Left,
    Right,
    Up,
    Down,
    Reveal,
    Flag,
    Reset,
}

impl Controller {
    fn new(config: Config) -> Controller {
        Controller {
            game: Game::new(config.width, config.height, config.mine_percent),
            debug: config.debugOn,
        }
    }

    fn get_action(&mut self) -> Result<Action, &str> {
        let event = read().unwrap();
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => Ok(Action::Left),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => Ok(Action::Right),
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => Ok(Action::Up),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => Ok(Action::Down),
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                ..
            }) => Ok(Action::Flag),
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => Ok(Action::Reveal),
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => Ok(Action::Reset),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                ..
            }) => {
                disable_raw_mode();
                process::exit(0);
            }
            _ => Err("this key doesn't do anything"),
        }
    }

    fn start(&mut self) {
        if self.debug {
            self.game.draw_debug();
        }
        loop {
            match self.receive_move_via_keyboard() {
                VictoryState::Over => {
                    self.show_menu(true);
                    break;
                }
                VictoryState::Won => {
                    self.show_menu(false);
                    break;
                }
                _ => (),
            }
        }
    }

    fn show_menu(&mut self, lost: bool) {
        if lost {
            print!("Game over! Try again? [y/n]: ");
        } else {
            print!("You won! Play again? [y/n]: ");
        }
        // let mut confirm = String::new();
        // stdin().read_line(&mut confirm).expect("failure");
        // if confirm.to_lowercase() == "y" {
        //     self.start();
        // } else {
        //     exit(0);
        // }
        exit(0);
    }

    fn receive_move_via_keyboard(&mut self) -> VictoryState {
        disable_raw_mode();
        if !self.debug {
            print!("{}[2J", 27 as char);
            crossterm::terminal::Clear(ClearType::All);
        }
        enable_raw_mode();
        self.game.draw();
        let action = self.get_action();
        if action.is_err() {
            return VictoryState::Continue;
        }
        match action.unwrap() {
            Action::Left => self.game.move_cursor(-1, 0),
            Action::Right => self.game.move_cursor(1, 0),
            Action::Up => self.game.move_cursor(0, -1),
            Action::Down => self.game.move_cursor(0, 1),
            Action::Reveal => self.game.click_at_cursor(),
            Action::Flag => self.game.flag_at_cursor(),
            _ => VictoryState::Continue,
        }
    }

    fn receive_move_via_tokens(&mut self) -> VictoryState {
        self.game.draw();
        let mut command = String::new();
        print!("Command: ");
        stdout().flush();
        stdin()
            .read_line(&mut command)
            .expect("failed to read command");
        println!();
        let tokens: Vec<&str> = command.split(' ').collect();
        if tokens.len() != 3 {
            println!("bad number of tokens. try again");
            return VictoryState::Continue;
        }
        let action = tokens[0];
        let x: i8 = tokens[1].trim().parse().expect("please type a number");
        let y: i8 = tokens[2].trim().parse().expect("please type a number");
        if action == "f" {
            self.game.flag(x, y)
        } else {
            self.game.click(x, y)
        }
    }
}
