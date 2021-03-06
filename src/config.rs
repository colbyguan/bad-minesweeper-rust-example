pub struct Config {
    pub width: i8,
    pub height: i8,
    pub mine_percent: i8,
    pub debugOn: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 8,
            height: 8,
            mine_percent: 5,
            debugOn: false,
        }
    }
}
