extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fs;

pub type Names = Vec<String>;

pub fn load_names(filename : &str) -> Names {

    let contents = fs::read_to_string(filename).expect("Could not read the file with the names!");
    let split = contents.split("\n");

    let names : Vec<&str> = split.collect::<Vec<&str>>();
    names.iter().filter( |name| !name.starts_with('#')).map( |&s| s.into()).collect()
}

pub fn load_shuffled_names(filename : &str) -> Names {
    let mut names : Names = load_names(filename);
    names.shuffle(&mut thread_rng());
    names.shuffle(&mut thread_rng());
    names.shuffle(&mut thread_rng());
    names
}
