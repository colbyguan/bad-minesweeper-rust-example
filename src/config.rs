pub struct Config {
    pub width: i8,
    pub height: i8,
    pub mine_percent: i8,
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 8,
            height: 8,
            mine_percent: 5,
        }
    }
}
