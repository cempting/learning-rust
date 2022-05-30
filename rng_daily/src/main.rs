
use std::time::Duration;

mod configuration;
mod names;
mod display;
mod font;

use clap::Parser;

/// Daily organizer with randomized list
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the peoples file
    #[clap(short, long)]
    people: String,

    /// Countdown timer value in seconds
    #[clap(short, long, default_value_t = 60)]
    countdown: u64,   
}

fn main() {
    let args = Args::parse();

    //let filename = "./src/names/people.txt";
    let names : names::Names = names::load_shuffled_names(args.people.as_str());
    let duration : Duration = Duration::from_secs(args.countdown);
    let font = font::Font::new();
    
    for name in names.iter() {
        display::countdown(&font, &name, &duration)
    }
}
