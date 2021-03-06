use std::{borrow::Borrow, collections::HashSet, fmt};

use rand::{thread_rng, Rng};

pub enum VictoryState {
    Continue,
    Over,
    Won,
}
#[derive(PartialEq, Clone, Debug)]
enum CellState {
    Revealed,
    Flagged,
    Hidden,
}

#[derive(PartialEq, Clone, Debug)]
enum CellType {
    Mine,
    Count(u8),
}

#[derive(Clone, Debug)]
struct Cell {
    state: CellState,
    cell_type: CellType,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self.cell_type {
            CellType::Count(c) => write!(f, "[{}] ", c),
            CellType::Mine => write!(f, "[*] "),
        };
    }
}

pub struct Game {
    width: i8,
    height: i8,
    mine_percent: i8,
    mine_count: i8,
    correct_flag_count: i8,
    grid: Box<[Cell]>,
}

impl Game {
    pub fn new(width: i8, height: i8, mine_percent: i8) -> Game {
        let mut game = Game {
            width: width,
            height: height,
            mine_percent,
            mine_count: 0,
            correct_flag_count: 0,
            grid: vec![
                Cell {
                    state: CellState::Hidden,
                    cell_type: CellType::Count(0),
                };
                width as usize * height as usize
            ]
            .into_boxed_slice(),
        };
        game.reset_mines_and_counts();
        game
    }

    pub fn draw(&self) {
        let mut x_axis = "    ".to_owned();
        for col in 0..self.width {
            x_axis.push_str(format!(" {:02} ", col).as_str());
        }
        println!("{}", x_axis);
        for row in 0..self.height {
            print!(" {:02} ", row);
            for col in 0..self.width {
                let cell = self.get_cell(col, row);
                match cell.state {
                    CellState::Flagged => print!("[f] "),
                    CellState::Hidden => print!("[ ] "),
                    CellState::Revealed => match cell.cell_type {
                        CellType::Count(c) => print!("[{}] ", c),
                        CellType::Mine => print!("[*] "),
                    },
                }
            }
            println!();
        }
    }

    pub fn draw_debug(&self) {
        let mut x_axis = "    ".to_owned();
        for col in 0..self.width {
            x_axis.push_str(format!(" {:02} ", col).as_str());
        }
        println!("{}", x_axis);
        for row in 0..self.height {
            print!(" {:02} ", row);
            for col in 0..self.width {
                let cell = self.get_cell(col, row);
                match cell.cell_type {
                    CellType::Count(c) => print!("[{}] ", c),
                    CellType::Mine => print!("[*] "),
                }
            }
            println!();
        }
    }

    pub fn reset_mines_and_counts(&mut self) {
        let mine_count = ((self.mine_percent as f64 / 100.0) * self.grid.len() as f64) as usize;
        self.mine_count = mine_count as i8;
        let mut rng = rand::thread_rng();
        let len = self.grid.len();
        for cell in self.grid.iter_mut() {
            if rng.gen_range(0..len) < mine_count {
                cell.cell_type = CellType::Mine;
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                println!("setting {} {}", x, y);
                self.set_count_adjacent_mines(x, y);
            }
        }
    }

    fn set_count_adjacent_mines(&mut self, x: i8, y: i8) {
        if self.get_cell(x, y).cell_type != CellType::Mine {
            let mut count = 0;

            for (ax, ay) in self.get_adjacents_slice(x, y).iter() {
                if self.is_oob(*ax, *ay) {
                    continue;
                }
                if self.get_cell(*ax, *ay).cell_type == CellType::Mine {
                    count += 1;
                }
            }
            self.get_cell_mut(x, y).cell_type = CellType::Count(count);
        }
    }

    fn get_adjacents_slice(&self, x: i8, y: i8) -> [(i8, i8); 8] {
        let above = y - 1;
        let below = y + 1;
        let left = x - 1;
        let right = x + 1;
        return [
            (left, above),
            (x, above),
            (right, above),
            (left, below),
            (x, below),
            (right, below),
            (left, y),
            (right, y),
        ];
    }
    fn get_cell(&self, x: i8, y: i8) -> &Cell {
        &self.grid[(x as usize + y as usize * self.width as usize) as usize]
    }

    fn get_cell_mut(&mut self, x: i8, y: i8) -> &mut Cell {
        &mut self.grid[(x as usize + y as usize * self.width as usize) as usize]
    }
    // is out of bounds
    fn is_oob(&self, ax: i8, ay: i8) -> bool {
        ax < 0 || ax >= self.width || ay < 0 || ay >= self.height
    }
    pub fn click(&mut self, x: i8, y: i8) -> VictoryState {
        if self.get_cell(x, y).cell_type == CellType::Mine {
            VictoryState::Over
        } else {
            self.reveal_from(x, y);
            VictoryState::Continue
        }
    }
    pub fn flag(&mut self, x: i8, y: i8) -> VictoryState {
        let cell = self.get_cell_mut(x, y);
        if cell.state != CellState::Flagged {
            cell.state = CellState::Flagged;
            if cell.cell_type == CellType::Mine {
                self.correct_flag_count += 1;
            }
        } else {
            cell.state = CellState::Hidden;
            if cell.cell_type == CellType::Mine {
                self.correct_flag_count -= 1;
            }
        }
        println!("{} {}", self.correct_flag_count, self.mine_count);
        if self.correct_flag_count == self.mine_count {
            VictoryState::Won
        } else {
            VictoryState::Continue
        }
    }
    fn reveal_from(&mut self, x: i8, y: i8) {
        let mut bfs = Vec::new();
        bfs.push((x, y));
        while !bfs.is_empty() {
            let (ax, ay) = bfs.remove(0);
            if self.is_oob(ax, ay) {
                continue;
            }
            let cell = self.get_cell(ax, ay);
            if cell.state == CellState::Revealed {
                continue;
            }
            if cell.cell_type == CellType::Count(0) {
                bfs.extend_from_slice(&self.get_adjacents_slice(ax, ay));
                let mut cell_mut = self.get_cell_mut(ax, ay);
                cell_mut.state = CellState::Revealed;
            } else {
                let mut cell_mut = self.get_cell_mut(ax, ay);
                cell_mut.state = CellState::Revealed;
            }
        }
    }
}
