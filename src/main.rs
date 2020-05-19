extern crate nix;

use nix::unistd::{fork, getpid, getppid, ForkResult};
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

        for word in line.split_whitespace() {
            println!("word '{}'", word);
        }

        let child_pid = match fork() {
            Ok(ForkResult::Child) => {
                println!(
                    "[child] I'm alive! My PID is {} and PPID is {}.",
                    getpid(),
                    getppid()
                );
                exit(0);
            }

            Ok(ForkResult::Parent { child, .. }) => {}
            Err(err) => {
                panic!("[main] fork() failed: {}", err);
            }
        };
    }
}
