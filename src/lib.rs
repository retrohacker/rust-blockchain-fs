extern crate sha2;
extern crate base64;

use std::fmt;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Error;
use std::io::ErrorKind;
use std::collections::HashSet;
use sha2::{Sha256, Digest};

#[derive(Debug)]
enum LogEntry {
    Mkdir { path: String },
//    Rename { from: String, to: String },
    Rmdir { path: String },
//    Unlink { path: String },
    Write { path: String, hash: String },
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LogEntry::Mkdir{ref path} => {
                write!(f, "{}", path)
            },
            &LogEntry::Rmdir{ref path} => {
                write!(f, "{}", path)
            },
            &LogEntry::Write{ref path, ref hash }  => {
                write!(f, "{} {}", path, hash)
            },
        }
    }
}

#[derive(Debug)]
pub struct Log {
    log: Vec<LogEntry>,
    pub name: String,
    pub blob_dir: String,
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let mut hash = String::from("");
        let mut res: Vec<String> = Vec::new();
        for entry in self.log.iter() {
            let mut hasher = Sha256::default();
            let mut t = "";
            match entry {
                &LogEntry::Mkdir {ref path} => t = "mkdir",
                &LogEntry::Rmdir {ref path} => t = "rmdir",
                &LogEntry::Write {ref path, ref hash} => t = "write",
            }
            if i == 0  {
                hasher.input(format!("{} {}", t, entry).as_bytes());
            } else {
                hasher.input(format!("{} {} {}", t, entry, hash).as_bytes());
            }
            hash = base64::encode_config(
                hasher.result().as_slice(),
                base64::Config::new(
                    base64::CharacterSet::UrlSafe,
                    false,
                    true,
                    base64::LineWrap::NoWrap
                )
            );
            res.push(format!("{} {} {}", hash, t, entry));
            i = i + 1;
        }
        for line in res.iter().rev() {
            writeln!(f, "{}", line)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
struct MetaData {
    empty: bool,
    is_file: bool,
    is_dir: bool,
    self_exists: bool,
    parent_exists: bool,
    dirname: String,
}

fn get_meta_data(log: &Vec<LogEntry>, p: &str) -> MetaData {
    let mut res = MetaData {
        empty: true,
        is_file: false,
        is_dir: false,
        self_exists: false,
        parent_exists: false,
        dirname: dirname(p),
    };

    let mut children = HashSet::new();

    for entry in log.iter() {
        match entry {
            &LogEntry::Mkdir{ref path} => {
                if *path == res.dirname {
                    res.parent_exists = true;
                }
                if *path == *p {
                    res.self_exists = true;
                    res.is_dir = true;
                }
                if path.starts_with(p) && path.len() != p.len() {
                    children.insert(path);
                }
            },
            &LogEntry::Rmdir{ref path} => {
                if *path == res.dirname {
                    res.parent_exists = false;
                }
                if *path == *p {
                    res.is_dir = false;
                    res.self_exists = false;
                }
                if path.starts_with(p) && path.len() != p.len() {
                    children.remove(path);
                }
            },
            _ => ()
        }
    }

    if !children.is_empty() {
        res.empty = false;
    }

    res
}

impl Log {
    pub fn mkdir(&mut self, p: &str) -> Result<(), Error> {
        /* Begin validation */

        // Ensure we were passed a string
        if p.len() == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Directory can not be an empty string!" )
            ));
        }

        // Ensure the directory is off of the root
        // We want the panic here, since the previous if condition ensures there
        // is an element, if we reach this line and next returns None, there is
        // a bug somewhere
        if p.chars().next().expect("No chars in path!") != '/' {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Directory path must start with '/'!" )
            ));
        }

        let info = get_meta_data(&self.log, p);

        if !info.parent_exists {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Directory {} does not exists!", info.dirname)
            ))
        }

        if info.self_exists {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Directory {} already exists!", p)
            ))
        }

        /* End validation */

        self.log.push(LogEntry::Mkdir { path: String::from(p) });

        Ok(())
    }
    pub fn rmdir(&mut self, p: &str) -> Result<(), Error> {
        /* Begin validation */

        // Ensure we were passed a string
        if p.len() == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Directory can not be an empty string!" )
            ));
        }

        // Ensure the directory is off of the root
        // We want the panic here, since the previous if condition ensures there
        // is an element, if we reach this line and next returns None, there is
        // a bug somewhere
        if p.chars().next().expect("No chars in path!") != '/' {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Directory path must start with '/'!" )
            ));
        }

        let info = get_meta_data(&self.log, p);

        if !info.self_exists {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Directory {} does not exists!", p)
            ))
        }

        if !info.empty {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Directory {} is not empty!", p)
            ))
        }

        /* End validation */
        self.log.push(LogEntry::Rmdir { path: String::from(p) });

        Ok(())
    }
    pub fn write(&mut self, p: &str, data: Vec<u8>) -> Result<(), Error> {
        /* Begin validation */

        if p.len() == 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Directory can not be an empty string!" )
            ));
        }

        if p.chars().next().expect("No chars in path!") != '/' {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Directory path must start with '/'!" )
            ));
        }

        let info = get_meta_data(&self.log, p);

        if !info.parent_exists {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Directory {} does not exists!", info.dirname)
            ))
        }

        if info.is_dir {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Directory {} already exists!", p)
            ))
        }

        /* TODO: diffing and patching */

        // Calculate the hash of the file we are writing
        let mut hasher = Sha256::default();
        hasher.input(&data);
        let hash = base64::encode_config(
                hasher.result().as_slice(),
                base64::Config::new(
                    base64::CharacterSet::UrlSafe,
                    false,
                    true,
                    base64::LineWrap::NoWrap
                )
            );

        // Calculate the path we are writing to
        let blob_path: PathBuf = [&self.blob_dir, &hash].iter().collect();

        // Write the data
        let mut f = File::create(blob_path)?;
        f.write_all(&data)?;

        self.log.push(LogEntry::Write { path: String::from(p), hash });
        Ok(())
    }
}

fn dirname(path: &str) -> String {
    let index = last_sep(path);
    let mut res = path.to_string();
    if index > 0 {
        res.truncate(index);
    } else {
        res = String::from("/");
    }
    res
}

fn last_sep(path: &str) -> usize {
    let mut chars = path.chars();
    let mut index = path.len() - 1;
    loop {
        if index == 0 || match chars.next_back() {
            Some(c) => c == '/',
            None => true,
        } {
            break;
        }
        index = index - 1;
    };
    index
}

pub fn new(name: &str, path: &str) -> Result<Log, Error> {
    let res = fs::create_dir(&path);
    match res {
        Err(ref error) => {
            match error.kind() {
                ErrorKind::AlreadyExists => (),
                _ => {
                    return Err(Error::new(
                        error.kind(),
                        "Can not save files to specified path"
                    ))
                },
            }
        },
        _ => (),
    }
    Ok(Log {
        // All filesystems have a root
        log: vec!(LogEntry::Mkdir { path: String::from("/") }),
        name: name.to_string(),
        blob_dir: path.to_string(),
    })
}
