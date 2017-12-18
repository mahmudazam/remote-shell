
use std::process::Command;

use built_ins::run_built_in;

static PATH : [&'static str; 4] =
    ["/bin/", "/usr/bin/", "/usr/local/bin/", "/usr/sbin/"];

fn try_exec(name : String, argv : Vec<&str>) -> i32 {
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

pub fn exec_comm(comm : String) -> i32 {
    /* Tokenize string and get the command name and arguments: */
    let mut comm_vec = comm.split_whitespace();
    let name = match comm_vec.next() {
        None => format!(""),
        Some(name) => format!("{}", name),
    };
    
    let mut argv = Vec::new();
    for i in comm_vec {
        argv.push(i);
    }

    /* Try running as a built-in: */
    let exit_status = run_built_in(name.clone(), argv.clone());

    /* Not a built-in: */
    if -1 == exit_status {
        return try_exec(name, argv);
    } else {
        return 0;
    }

}

