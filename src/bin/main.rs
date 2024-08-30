use std::env;

use fre::Fre;

fn main() {
    let args = env::args();
    let fre = Fre::from(args);
    fre.execute();
}
