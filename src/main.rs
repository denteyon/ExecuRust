use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    while true {
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("there was no next line")
            .expect("the line could not be read");

        println!("{}", line)
    }
}
