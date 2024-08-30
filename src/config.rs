#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub recursive: bool,
    pub recursive_full: bool,
    pub edit: bool,
    pub delete: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            recursive: false,
            recursive_full: false,
            edit: false,
            delete: false,
        }
    }
}
