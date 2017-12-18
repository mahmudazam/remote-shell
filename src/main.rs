
extern crate libc;

use std::io::{self, Write};
use std::process;
use std::process::Command;

mod built_ins;
use built_ins::run_built_in;

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
        
        return 0;
    } else {
        return 0;
    }

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

