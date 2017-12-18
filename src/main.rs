
extern crate libc;

use std::io::{self, Write};
use std::process::Command;
use std::env;

mod built_ins;
use built_ins::run_built_in;

fn nop() {
    return;
}

fn print_prompt() {
    let cwd = env::current_dir().unwrap();
    let cwd = format!("{}", cwd.display());
    print!("\n::{}\n> ", cwd);
    io::stdout().flush()
        .expect("Printing failed");
}

fn get_comm() -> String {
    let mut comm = String::new();
    match io::stdin().read_line(&mut comm) {
        Ok(0)  => {
          run_built_in(String::from("/bin/exit"), Vec::new());
        },
        Ok(_)  => nop(),
        Err(_) => nop(),
    }
    return comm;
}

fn exec_comm(comm : String) -> i32 {
    /* Tokenize string and get the command name and arguments: */
    let mut comm_vec = comm.split_whitespace();
    let path = match comm_vec.next() {
        None => format!(""),
        Some(name) => format!("/bin/{}", name),
    };
    let mut argv = Vec::new();
    for i in comm_vec {
        argv.push(i);
    }
    
    let exit_status = run_built_in(path.clone(), argv.clone());
    if -1 == exit_status {
        /* Set up and run child: */
        let mut child = Command::new(path)
                            .args(argv)
                            .spawn()
                            .expect("Spawn failure");
        let exit_status = child.wait()
                          .expect("wait failure");
        
        return match exit_status.code() {
            None => -1,
            Some(c) => c,
        };
    } else {
        return 0;
    }

}

fn main() {
    loop {
        print_prompt();
        let comm = get_comm();
        exec_comm(comm);
    }
}

