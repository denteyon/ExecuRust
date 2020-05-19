extern crate nix; //unix lib

use nix::unistd::{fork, ForkResult};
use std::io::{self, BufRead};
use std::process::exit;
use std::process::Command;

fn main() {
    let stdin = io::stdin(); //input
    loop {
        // infinite loop
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("there was no next line")
            .expect("the line could not be read");

        let mut v: Vec<&str> = line.split(' ').collect(); //input line

        let str: &str = v[0]; //first argument

        v.remove(0); //remove first argument

        let _ = match fork() {
            // child
            Ok(ForkResult::Child) => {
                // exec
                Command::new(str)
                    .args(&v)
                    .spawn()
                    .expect("command failed to start");
                exit(0);
            }

            // parent
            Ok(ForkResult::Parent { child, .. }) => {}

            //error
            Err(err) => {
                panic!("[main] fork() failed: {}", err);
            }
        };
    }
}
