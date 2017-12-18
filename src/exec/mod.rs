
use std::process::Command;

use built_ins::run_built_in;

static PATH : [&'static str; 4] =
    ["/bin/", "/usr/bin/", "/usr/local/bin/", "/usr/sbin/"];

fn try_exec(name : String, argv : Vec<String>) -> i32 {
    for i in PATH.iter() {
        /* Try an exec: */
        let p = format!("{}{}", i, name);
        let child = Command::new(p)
                            .args(&argv)
                            .spawn();
        match child {
            Ok(mut c) => {
                let exit_status = c.wait()
                    .expect("Wait failure");
                return match exit_status.code() {
                    None => -1,
                    Some(s) => s,
                };
            },
            Err(_) => {
                ; /* Continue if failed */
            },
        }
    }

    println!("-shell: command not found");
    return -1;
}

fn path_and_args(comm_str : String) -> (String, Vec<String>) {
    let mut comm_vec = comm_str.split_whitespace();
    let name = String::from(match comm_vec.next() {
        None => format!(""),
        Some(name) => format!("{}", name),
    });
    
    let mut argv = Vec::new();
    for i in comm_vec {
        argv.push(String::from(i));
    }

    return (name, argv);
}

/*
fn get_comms(comm : String) -> Vec<Command> {
    let mut comm_strs = comm.split('|').collect();
    let mut comms = Vec::new();
    for i in comms {
        let child = Command::new();
    }
    return comms;
}
*/

pub fn exec_comm(comm : String) -> i32 {
    /* Tokenize string and get the command name and arguments: */
    let command = path_and_args(comm);

    /* Try running as a built-in: */
    let exit_status = run_built_in(&(command.0), &(command.1));

    /* Not a built-in: */
    if -1 == exit_status {
        return try_exec(command.0, command.1);
    } else {
        return 0;
    }
}

