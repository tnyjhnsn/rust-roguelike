use std::fs::File;
use super::{MyEntities};

use ron::de::from_reader;

pub fn load_raws() {
    let file = File::open("raws/spawns.ron").expect("Cannot open file");
    let entities: MyEntities = from_reader(file).expect("Cannot read from file");
    println!("{:?}", entities);
}
