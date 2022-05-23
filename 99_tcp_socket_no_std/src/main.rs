extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

mod server;
mod client;

fn main() {
    let mut verbose = false;
    let mut mode = "".to_string();
    let mut address = "127.0.0.1".to_string();
    let mut port = "8080".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("TCP socket demo.");
        ap.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut mode).add_option(&["-m", "--mode"], Store, "The mode (Server | Client)");
        ap.refer(&mut address).add_option(&["-a", "--address"], Store, "The server IP address");        
        ap.refer(&mut port).add_option(&["-p", "--port"], Store, "The server port");        
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("The mode is {}", mode);
        println!("{} {}:{}", if mode=="Server" {"Creating a server for"} else {"Connecting to server"}, address, port);
    }

    match &mode as &str {
        "server" => println!("Starting Server ..."),
        "client" => println!("Starting Client ..."),
        _ => panic!("Expected Server or Client as input for the mode."),
    }
}
