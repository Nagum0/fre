pub mod config;
pub mod utils;

use config::Config;
use std::{env, ffi::OsString};

#[derive(Debug)]
pub struct Fre {
    path: OsString,
    pattern: String,
    replace: String,
    config: Config,
}

impl Fre {
    pub fn new() -> Fre {
        Fre {
            path: OsString::new(),
            pattern: String::new(),
            replace: String::new(),
            config: Config::new(),
        }
    }

    pub fn from(mut args: env::Args) -> Fre {
        // Skip executable name:
        args.next();

        let mut fre = Fre::new();
        let mut arg_counter: usize = 0;

        for arg in args {
            // Setting up the options:
            if arg.starts_with('-') {
                match arg.as_str() {
                    "-r" => fre.config.recursive = true,
                    "-rf" => fre.config.recursive_full = true,
                    "-e" => fre.config.edit = true,
                    "-d" => fre.config.delete = true,
                    _ => panic!("Unknown flag: {}", arg),
                }
            }

            if arg_counter == 0 {
                fre.pattern = arg;
                arg_counter += 1;
            } else if arg_counter == 1 {
                fre.replace = arg;
                arg_counter += 1;
            } else if arg_counter == 2 {
                fre.path = OsString::from(arg);
                arg_counter += 1;
            }
        }

        // Check if correct amount of arguments were passed:
        if arg_counter != 3 {
            panic!("Expected 3 arguments but received: {}", arg_counter);
        }

        fre
    }

    pub fn execute(&self) {
        // Recursive full mode:
        if self.config.recursive_full {
            self.recursive_full_mode();
        }
        // Recursive mode:
        else if self.config.recursive {
            self.recursive_mode();
        // Single mode:
        } else {
            utils::transform_file_contents(
                &self.path,
                &self.pattern,
                &self.replace,
                self.config.edit,
                self.config.delete,
            );
        }
    }

    fn recursive_mode(&self) {
        let xs = utils::collect_files(&self.path, false);
        for x in xs {
            println!("{:?}", x);
        }
    }

    fn recursive_full_mode(&self) {
        todo!("Recursive full mode not implemented!");
    }
}
