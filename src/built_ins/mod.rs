
use std::path::Path;
use std::env;

pub fn run_built_in(path : String, argv : Vec<&str>) -> (i32) {
    return match path.as_ref() {
        "/bin/cd" => cd(argv),
        "/bin/pwd" => pwd(),
        "/bin/exit" => exit(),
        _ => -1,
    }
}

fn cd(argv : Vec<&str>) -> i32 {
    if 0 >= argv.len() {
        return -1;
    }
    let cwd = Path::new(argv[0]);
    let status = env::set_current_dir(&cwd);
    if status.is_ok() {
        return 0;
    } else {
        return -1;
    }
}

fn pwd() -> i32 {
    let cwd = env::current_dir().unwrap();
    println!("{}", cwd.display());
    return 0;
}

fn exit() -> i32 {
    println!("exit called");
    return 0;
}

