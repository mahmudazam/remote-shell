
use std::process::{Command};
use std::fs;
use std::os::unix::fs::PermissionsExt;

use built_ins::run_built_in;

static PATH : [&'static str; 4] =
    ["/bin/", "/usr/bin/", "/usr/local/bin/", "/usr/sbin/"];

fn path_and_args(comm_str : &str) -> (String, Vec<String>) {
    let comm_vec : Vec<&str> = comm_str
        .trim()
        .split_whitespace()
        .collect();

    let name = String::from(match comm_vec.get(0) {
        None => "",
        Some(s) => s,
    });

    let argv : Vec<String> = match comm_vec.get(1..comm_vec.len()) {
        None => Vec::new(),
        Some(a) => a.iter()
            .map(|x|
                String::from(*x)
            ).collect(),
    };
    
    return (name, argv);
}

fn get_comm_strings(comm : String) -> Vec<(String, Vec<String>)> {
    comm.split('|').map(path_and_args).collect()
}

fn which(name : &String) -> Option<String> {
    for i in PATH.iter() {
        let p = format!("{}{}", i, name);
        match fs::metadata(&p) {
            Ok(m) => {
                if m.is_file() // regular file
                    && m.permissions().mode() & 0o111 != 0 { // executable
                    return Some(p);
                } else {
                    ; // continue
                }
            },
            Err(_) => {
                ; // continue
            },
        }
    }
    return None;
}

fn run_command(commands : Vec<(String, Vec<String>)>) -> i32 {
    let mut ret = -1;

    for i in commands {
        let path = match which(&(i.0)) {
            None => {
              return -1;
            },
            Some(s) => s,
        };
        let child = Command::new(&path)
            .args(&(i.1))
            .spawn();
        ret = match child {
            Ok(mut c) => {
                let exit_status = c.wait()
                    .expect("Wait failure");
                match exit_status.code() {
                    None => -1,
                    Some(s) => s,
                }
            },
            Err(_) => -1,
        };
    }
    return ret;
}

pub fn exec_comm(comm : String) -> i32 {
    /* Tokenize string and get the command name and arguments: */
    let command = get_comm_strings(comm);

    /* Try running as a built-in: */
    let exit_status = run_built_in(&(command[0].0), &(command[0].1));

    /* Not a built-in: */
    if -1 == exit_status {
        return run_command(command);
    } else {
        return 0;
    }
}

