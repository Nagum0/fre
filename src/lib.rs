pub mod config;
pub mod fre_error;
pub mod utils;

use config::Config;
use fre_error::FreError;
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

    pub fn from(mut args: env::Args) -> Result<Fre, FreError> {
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
                    _ => return Err(FreError::UnknownFlagError(arg)),
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
            return Err(FreError::ArgError(3, arg_counter));
        }

        Ok(fre)
    }

    pub fn execute(&self) -> Result<(), FreError> {
        // Recursive (-r) or recursive full (-rf) mode:
        if self.config.recursive || self.config.recursive_full {
            self.recursive_mode()?;
        }
        // Single mode:
        else {
            match utils::transform_file_contents(
                &self.path,
                &self.pattern,
                &self.replace,
                self.config.edit,
                self.config.delete,
            ) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }

        Ok(())
    }

    fn recursive_mode(&self) -> Result<(), FreError> {
        let file_paths = utils::collect_files(&self.path, self.config.recursive_full)?;

        for file_path in file_paths {
            if !self.config.edit {
                println!("{:?}:", file_path);
            }

            match utils::transform_file_contents(
                &file_path,
                &self.pattern,
                &self.replace,
                self.config.edit,
                self.config.delete,
            ) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            };
        }

        Ok(())
    }
}
