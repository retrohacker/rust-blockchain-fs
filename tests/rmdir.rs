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
    let mut log = new("rmdir", "./rmdir").unwrap();
    log.mkdir("/foo");
    log.mkdir("/foo/bar");
    log.mkdir("/foo/buzz");
    Harness { log }
}

#[test]
fn rmdir() {
    let mut harness = setup();
    let err = harness.log.rmdir("/foo/buzz");
    assert!(
        err.is_none(),
        "rmdir returned an error {}", err.expect("is_none with value"));
}

#[test]
fn rmdir_missing() {
    let mut harness = setup();
    let err = harness.log.rmdir("/beep/bop");
    assert!(
        err.is_some(),
        "rmdir did not return an error!");
}

#[test]
fn rmdir_empty_string() {
    let mut harness = setup();
    let err = harness.log.rmdir("");
    assert!(
        err.is_some(),
        "rmdir did not return an error!");
}

#[test]
fn rmdir_invalid_string() {
    let mut harness = setup();
    let err = harness.log.rmdir("C:\\\\not\\valid");
    assert!(
        err.is_some(),
        "rmdir did not return an error!");
}

#[test]
fn rmdir_double() {
    let mut harness = setup();
    let err = harness.log.rmdir("/foo/bar");
    assert!(
        err.is_none(),
        "rmdir returned an error on first invocation!");
    let err = harness.log.rmdir("/foo/bar");
    assert!(
        err.is_some(),
        "rmdir did not return an error! on second invocation");
}

#[test]
fn rmdir_not_empty() {
    let mut harness = setup();
    let err = harness.log.rmdir("/foo");
    assert!(
        err.is_some(),
        "rmdir did not return an error!");
}
