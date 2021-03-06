use crossterm::{execute, style::Print};
use rand::Rng;
use std::{fmt, io::stdout};

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
    cursor_x: i8,
    cursor_y: i8,
    width: i8,
    height: i8,
    mine_percent: i8,
    mine_count: i8,
    correct_flag_count: i8,
    grid: Box<[Cell]>,
}

// Emoji keycaps 1 to 8
const EMOJI_NUMBERS: [&'static str; 9] = [
    "  ",
    "\u{0031}\u{FE0F}\u{20E3}",
    "\u{0032}\u{FE0F}\u{20E3}",
    "\u{0033}\u{FE0F}\u{20E3}",
    "\u{0034}\u{FE0F}\u{20E3}",
    "\u{0035}\u{FE0F}\u{20E3}",
    "\u{0036}\u{FE0F}\u{20E3}",
    "\u{0037}\u{FE0F}\u{20E3}",
    "\u{0038}\u{FE0F}\u{20E3}",
];
const STRING_NUMBERS: [&'static str; 9] = ["  ", "1 ", "2 ", "3 ", "4 ", "5 ", "6 ", "7 ", "8 "];
const TERM_BG_WHITE: &'static str = "\x1b[48;5;240m";
const TERM_BG_YELLOW: &'static str = "\x1b[48;5;242m";
const TERM_RESET: &'static str = "\x1b[0m";

impl Game {
    pub fn new(width: i8, height: i8, mine_percent: i8) -> Game {
        let mut game = Game {
            cursor_x: width / 2,
            cursor_y: height / 2,
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

    pub fn move_cursor(&mut self, dx: i8, dy: i8) -> VictoryState {
        // Wrap any out of bounds
        self.cursor_x = (self.cursor_x + dx + self.width) % self.width;
        self.cursor_y = (self.cursor_y + dy + self.height) % self.height;
        // 7 + 1 + 8 % 8
        VictoryState::Continue
    }

    pub fn draw(&self) {
        // let mut x_axis = "    ".to_owned();
        // for col in 0..self.width {
        //     x_axis.push_str(format!(" {:02} ", col).as_str());
        // }
        // println!("{}", x_axis);
        for row in 0..self.height {
            // print!(" {:02} ", row);
            for col in 0..self.width {
                let cell = self.get_cell(col, row);
                if row == self.cursor_y && col == self.cursor_x {
                    self.raw_print(TERM_BG_YELLOW);
                } else if row == self.cursor_y || col == self.cursor_x {
                    self.raw_print(TERM_BG_WHITE);
                }
                match cell.state {
                    CellState::Flagged => self.raw_print("⛳"),
                    CellState::Hidden => self.raw_print("🔲"),
                    CellState::Revealed => match cell.cell_type {
                        CellType::Count(c) => self.raw_print(STRING_NUMBERS[c as usize]),
                        CellType::Mine => self.raw_print("💣"),
                    },
                }
                if row == self.cursor_y || col == self.cursor_x {
                    self.raw_print(TERM_RESET);
                }
            }
            self.raw_print("\r\n");
        }
        self.raw_print("\r\n");
        self.raw_print("\r\n");
    }

    fn raw_print(&self, s: &str) {
        execute!(stdout(), Print(s));
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
        let mut rng = rand::thread_rng();
        let len = self.grid.len();
        for cell in self.grid.iter_mut() {
            if rng.gen_range(0..len) < mine_count {
                cell.cell_type = CellType::Mine;
                self.mine_count += 1;
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

    pub fn click_at_cursor(&mut self) -> VictoryState {
        self.click(self.cursor_x, self.cursor_y)
    }

    pub fn click(&mut self, x: i8, y: i8) -> VictoryState {
        if self.get_cell(x, y).cell_type == CellType::Mine {
            VictoryState::Over
        } else {
            self.reveal_from(x, y);
            VictoryState::Continue
        }
    }

    pub fn flag_at_cursor(&mut self) -> VictoryState {
        self.flag(self.cursor_x, self.cursor_y)
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
