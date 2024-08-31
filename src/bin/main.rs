use std::{env, process::exit};

use fre::Fre;

fn main() {
    let args = env::args();

    let fre = match Fre::from(args) {
        Ok(fre) => fre,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    match fre.execute() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}
