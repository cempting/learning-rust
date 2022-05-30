
use std::time::Duration;

mod configuration;
mod names;
mod display;
mod font;

//use clap::Parser;

fn main() {
    let filename = "./src/names/people.txt";
    let names : names::Names = names::load_shuffled_names(filename);
    let duration : Duration = Duration::from_secs(3);
    let font = font::Font::new();
    
    for name in names.iter() {
        display::countdown(&font, &name, &duration)
    }
}
