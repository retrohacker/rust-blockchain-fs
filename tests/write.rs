extern crate blox;
use blox::*;

struct Harness{
    log: Log
}

impl Drop for Harness {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.log.blob_dir);
    }
}

fn setup() -> Harness {
    let mut log = new("write", "./write").unwrap();
    log.mkdir("/foo");
    log.mkdir("/foo/bar");
    log.mkdir("/foo/buzz");
    Harness { log }
}

#[test]
fn write() {
    let mut harness = setup();
    let content = String::from("hello world").into_bytes();
    let err = harness.log.write("/foo/beep.txt", content).err();
    assert!(
        err.is_none(),
        "write returned an error {}", err.expect("is_none with value"));
}
