
use std::io::{self, Write};
use std::env;

mod built_ins;
use built_ins::run_built_in;

mod exec;
use exec::exec_comm;

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

fn main() {
    loop {
        print_prompt();
        let comm = get_comm();
        exec_comm(comm);
    }
}

