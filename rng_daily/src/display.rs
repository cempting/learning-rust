extern crate termion; 

use std::io::{stdin, Read};
use std::time::Duration;
use termion::{color};

use crate::font;

fn pause(text : &str) {
    println!("{}", text);
    stdin().read(&mut [0]).unwrap();
}

fn print_timer(font : &font::Font, countdown : &std::time::Duration) {
    print!("{}", termion::cursor::Goto(1, 10));

    match countdown.as_secs() {
        10..=19 => print!("{}", color::Fg(color::Yellow)),
        0..=9 => print!("{}", color::Fg(color::Red)),
        _ => print!("{}", color::Fg(color::Green)),
    }

    let min = countdown.as_secs() / 60;
    let sec = countdown.as_secs() % 60;
    font.print(format!("{:0>2}:{:0>2}  ", min, sec).as_str());
}

pub fn countdown(font : &font::Font, name : &String, duration : &std::time::Duration) {

    print!("{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), color::Fg(color::LightCyan));
    font.print(format!("{}", name).as_str());
    pause("Du bist dran ...");

    let mut countdown = *duration;
    let interval = Duration::from_secs(1);
    loop {
        countdown -= interval;

        print_timer(&font, &countdown);
        
        std::thread::sleep(interval);
        if countdown <= Duration::from_secs(0) {
            pause("Danke ...");
            return
        }
    }
}
