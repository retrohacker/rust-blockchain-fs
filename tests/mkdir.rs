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
    let mut log = new("mkdir", "./mkdir").unwrap();
    log.mkdir("/foo");
    log.mkdir("/foo/bar");
    log.mkdir("/foo/buzz");
    Harness { log }
}

#[test]
fn mkdir() {
    let mut harness = setup();
    let err = harness.log.mkdir("/foo/buzz/bazz");
    assert!(
        err.is_none(),
        "mkdir returned an error {}", err.expect("is_none with value"));
}

#[test]
fn mkdir_duplicate() {
    let mut harness = setup();
    let err = harness.log.mkdir("/foo/buzz");
    assert!(
        err.is_some(),
        "mkdir did not return an error!");
}

#[test]
fn mkdir_no_parent() {
    let mut harness = setup();
    let err = harness.log.mkdir("/beep/bop");
    assert!(
        err.is_some(),
        "mkdir did not return an error!");
}

#[test]
fn mkdir_empty_string() {
    let mut harness = setup();
    let err = harness.log.mkdir("");
    assert!(
        err.is_some(),
        "mkdir did not return an error!");
}

#[test]
fn mkdir_invalid_string() {
    let mut harness = setup();
    let err = harness.log.mkdir("C:\\\\not\\valid");
    assert!(
        err.is_some(),
        "mkdir did not return an error!");
}

#[test]
fn mkdir_after_rmdir() {
    let mut harness = setup();
    let err = harness.log.rmdir("/foo/buzz");
    assert!(
        err.is_none(),
        "rmdir returned an error {}", err.expect("is_none with value"));
    let err = harness.log.mkdir("/foo/buzz");
    assert!(
        err.is_none(),
        "mkdir returned an error {}", err.expect("is_none with value"));
}
