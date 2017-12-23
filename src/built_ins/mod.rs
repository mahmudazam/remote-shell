
use std::path::Path;
use std::env;

pub fn run_built_in(path : &String, argv : &Vec<String>,
        buf : &mut String) -> (i32) {
    return match path.as_ref() {
        "cd" => cd(argv),
        "pwd" => pwd(buf),
        "exit" => exit(buf),
        _ => -1,
    }
}

fn cd(argv : &Vec<String>) -> i32 {
    if 0 >= argv.len() {
        return -1;
    }
    let cwd = Path::new(&(argv[0]));
    let status = env::set_current_dir(&cwd);
    if status.is_ok() {
        return 0;
    } else {
        return -1;
    }
}

fn pwd(buf : &mut String) -> i32 {
    let cwd = env::current_dir().unwrap();
    *buf = format!("{}\n", cwd.display());
    return 0;
}

fn exit(buf : &mut String) -> i32 {
    *buf = format!("exiting remote-shell");
    return -3;
}

