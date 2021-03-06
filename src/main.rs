mod config;
mod game;

use std::{
    io::{stdin, stdout, Read, Write},
    process::exit,
};

use config::Config;
use game::{Game, VictoryState};

fn main() {
    println!("minesweeper time");
    let mut controller = Controller::new(Config::new());
    controller.start();
}

struct Controller {
    game: Game,
}

impl Controller {
    fn new(config: Config) -> Controller {
        Controller {
            game: Game::new(config.width, config.height, config.mine_percent),
        }
    }

    fn start(&mut self) {
        self.game.draw_debug();
        loop {
            match self.receive_move() {
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

    fn receive_move(&mut self) -> VictoryState {
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
