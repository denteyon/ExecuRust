extern crate nix;

use nix::unistd::{fork, ForkResult};
use std::io::{self, BufRead};
use std::process::exit;
use std::process::Command;

fn main() {
    let stdin = io::stdin();
    loop {
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("there was no next line")
            .expect("the line could not be read");

        let mut v: Vec<&str> = line.split(' ').collect();
        let str: &str = v[0];
        v.remove(0);

        let _ = match fork() {
            Ok(ForkResult::Child) => {
                Command::new(str)
                    .args(&v)
                    .spawn()
                    .expect("command failed to start");
                exit(0);
            }

            Ok(ForkResult::Parent { child, .. }) => {}
            Err(err) => {
                panic!("[main] fork() failed: {}", err);
            }
        };
    }
}
