
use std::process::Command;

use built_ins::run_built_in;

static PATH : [&'static str; 4] =
    ["/bin/", "/usr/bin/", "/usr/local/bin/", "/usr/sbin/"];

fn try_exec(name : &String, argv : &Vec<String>) -> i32 {
    for i in PATH.iter() {
        /* Try an exec: */
        let p = format!("{}{}", i, name);
        let child = Command::new(p)
                            .args(argv)
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

    if name.len() != 0 {
        println!("-shell: command not found");
    }
    return -1;
}

fn path_and_args(comm_str : &str) -> (String, Vec<String>) {
    let comm_vec : Vec<&str> = comm_str.split_whitespace().collect();
    let name = String::from(match comm_vec.get(0) {
        None => "",
        Some(s) => s,
    });
    let argv : Vec<String> = match comm_vec.get(1..comm_vec.len()) {
        None => Vec::new(),
        Some(a) => a.iter().map(
            |x| String::from(*x)
        ).collect(),
    };
    
    return (name, argv);
}

fn get_comms(comm : String) -> Vec<(String, Vec<String>)> {
    comm.split('|').map(path_and_args).collect()
}

pub fn exec_comm(comm : String) -> i32 {
    /* Tokenize string and get the command name and arguments: */
    let command = get_comms(comm);

    /* Try running as a built-in: */
    let exit_status = run_built_in(&(command[0].0), &(command[0].1));

    /* Not a built-in: */
    if -1 == exit_status {
        return try_exec(&(command[0].0), &(command[0].1));
    } else {
        return 0;
    }
}

