
extern crate libc;

use std::io::{self, Write};
use std::process;
use libc::EOF;

fn nop() {
    return;
}

fn exit_shell(i : i32) {
    process::exit(i);
}

fn main() {
    loop {
        print!("<{}> > ", "cwd");
        io::stdout().flush()
            .expect("Printing failed");
        /* Get command from standard input: */
        let mut comm = String::new();

        match io::stdin().read_line(&mut comm) {
            Ok(n)  => if(0 == n) {
                exit_shell(1);
            },
            Err(_) => nop(),
        }
        print!("Command to execute: {}", comm);

        /* Exit shell: */
        if false {
            exit_shell(0);
        }
    }
}

