extern crate blox;
use blox::*;

fn setup() -> Log {
    let mut log = new("mkdir", "./mkdir").unwrap();
    log.mkdir("/foo").unwrap();
    log.write("/foo/beep.txt", "hello world!".as_bytes().to_vec()).unwrap();
    log.mkdir("/foo/bar").unwrap();
    log.rmdir("/foo/bar").unwrap();
    log.mkdir("/foo/bar").unwrap();
    log.mkdir("/foo/buzz").unwrap();
    log
}

fn main() {
    let mut log = setup();
    println!("{}", log);
}
