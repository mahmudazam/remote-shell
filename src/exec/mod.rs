
use std::process::Command;

use built_ins::run_built_in;

pub fn exec_comm(comm : String) -> i32 {
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

