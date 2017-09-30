extern crate discord;
extern crate libc;

use std::fs;
use std::path::{Path, PathBuf};

mod interface;
mod io;
mod net;

fn main() {

    let pb = PathBuf::from(Path::new("output"));
    let mut di = interface::DiState::new(pb).unwrap();

    match net::init_discord() {
        Ok(d) => match d.connect() {
            Ok((conn, re)) => net::event_loop(&mut di, conn, re),
            Err(_) => println!("failed to connect")
        },
        Err(net::InitError(r)) => println!("failed to initialize: {}", r)
    }

}
