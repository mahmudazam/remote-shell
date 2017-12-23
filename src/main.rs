
use std::io::prelude::*;
use std::io::{Write, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::env;

mod built_ins;

mod exec;
use exec::exec_comm;

fn nop() {
    return;
}

fn get_prompt() -> String {
    let cwd = env::current_dir().unwrap();
    let cwd = format!("::{} >\0", cwd.display());
    return cwd;
}

fn reply(buf : &mut String, writer : &mut BufWriter<&TcpStream>) {
    writer.write_all(format!("{}\n{}", buf, get_prompt()).as_bytes())
        .expect("Reply failed");
    writer.flush().expect("");
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:60000")
        .unwrap();
    for stream in server.incoming() {
        match stream {
            Err(_) => {
                continue;
            },
            Ok(s) => {
                let mut reader : BufReader<&TcpStream>
                    = BufReader::new(&s);
                let mut writer : BufWriter<&TcpStream>
                    = BufWriter::new(&s);

                reply(&mut String::from("# Connected to server"), &mut writer);

                loop {
                    let mut comm = String::new();
                    match reader.read_line(&mut comm) {
                        Ok(0) => { break; },
                        _ => nop(),
                    };
                    print!("Command received: {}", comm);

                    let mut buf = String::new();
                    let status = exec_comm(comm, &mut buf);
                    if -3 == status {
                        break; // exit command
                    }
                    reply(&mut buf, &mut writer);
                }
            },
        }
    }
}

