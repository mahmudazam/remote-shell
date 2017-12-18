
extern crate libc;

use std::io::{self, Write};
use std::process;

fn nop() {
    return;
}

fn exit_shell(i : i32) {
    process::exit(i);
}

fn print_prompt() {
    print!("<{}> > ", "cwd");
    io::stdout().flush()
        .expect("Printing failed");
}

fn get_comm() -> String {
    let mut comm = String::new();
    match io::stdin().read_line(&mut comm) {
        Ok(0)  => exit_shell(0),
        Ok(_)  => nop(),
        Err(_) => nop(),
    }
    return comm;
}

fn exec_comm(comm : String) -> i32 {
    let comm_vec = comm.split_whitespace();
    let j = 0;
    for i in comm_vec {
        println!("Command token {}: {}", j, i);
    }
    return -1;
}

fn main() {
    loop {
        print_prompt();
        let comm = get_comm();
        exec_comm(comm);

        /* Exit shell: */
        if false {
            exit_shell(0);
        }
    }
}

