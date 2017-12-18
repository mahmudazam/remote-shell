
use std::process::Command;
use std::fs;

use built_ins::run_built_in;

static PATH : [&'static str; 4] =
    ["/bin/", "/usr/bin/", "/usr/local/bin/", "/usr/sbin/"];

fn which(name : String) -> Option<String> {
    for i in PATH.iter() {
        let meta = fs::metadata(format!("{}{}", i, name));
        match meta {
            Ok(_) => {
                return Some(format!("{}{}", i, name));
            },
            Err(_) => {;},
        }
    }
    return None;
}

pub fn exec_comm(comm : String) -> i32 {
    /* Tokenize string and get the command name and arguments: */
    let mut comm_vec = comm.split_whitespace();
    let path = match comm_vec.next() {
        None => format!(""),
        Some(name) => format!("{}", name),
    };
    
    let mut argv = Vec::new();
    for i in comm_vec {
        argv.push(i);
    }

    /* Try running as a built-in: */
    let exit_status = run_built_in(path.clone(), argv.clone());

    /* Not a built-in: */
    if -1 == exit_status {
        let path = which(path);
        match path {
            None => {
                println!("-shell: command not found");
                return -1;
            },
            Some(p) => {
                /* Set up and run child: */
                let mut child = Command::new(p)
                                    .args(argv)
                                    .spawn()
                                    .expect("Spawn failure");
                let exit_status = child.wait()
                                  .expect("wait failure");
                
                return match exit_status.code() {
                    None => -1,
                    Some(c) => c,
                };
            }
        }
    } else {
        return 0;
    }

}

