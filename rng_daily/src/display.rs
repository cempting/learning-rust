extern crate termion; 

use std::io;
use std::io::{stdin, Read};
use std::time::Duration;
use std::thread;

use crossbeam_channel::{bounded, select, tick, Receiver};
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

use termion::{color};

use crate::font;

fn pause(text : &str) {
    println!("{}", text);
    stdin().read(&mut [0]).unwrap();
}

fn sigint_notifier() -> io::Result<Receiver<()>> {
    let (s, r) = bounded(100);
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for _ in signals.forever() {
            if s.send(()).is_err() {
                break;
            }
        }
    });

    Ok(r)
}

fn print_timer(font : &font::Font, countdown : &std::time::Duration) {
    print!("{}", termion::cursor::Goto(1, 10));

    match countdown.as_secs() {
        10..=19 => print!("{}", color::Fg(color::LightYellow)),
        0..=9 => print!("{}", color::Fg(color::LightRed)),
        _ => print!("{}", color::Fg(color::LightGreen)),
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
    let ctrl_c_events = sigint_notifier().unwrap();
    let ticks = tick(interval);
   
    loop {
        select! {
            recv(ticks) -> _ => {
                countdown -= interval;
                print_timer(&font, &countdown);
                if countdown <= Duration::from_secs(0) {
                    pause("Danke ...");
                    break
                }
            }
            recv(ctrl_c_events) -> _ => {
                pause("Nächster ...");
                break;
            }
        }
    }
}
