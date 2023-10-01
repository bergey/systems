use std::fs::File;
use std::sync::{Arc, Mutex};

use std::{
    io::{prelude::*, BufReader},
};
use regex::Regex;

pub const FILENAME: &str = "access_counts";
pub const DOCSIZE: usize = 16;

pub type FileMutex = Arc<Mutex<File>>;
pub struct Doc(Vec<u8>);

impl Doc {
    pub fn new(v: Vec<u8>) -> Doc {
        Doc(v)
    }

    pub fn fill(byte: u8) -> Doc {
        Doc(vec![byte; DOCSIZE])
    }

    pub fn as_byte(&self) -> u8 {
        self.0[0]
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

pub fn http_reply(code: u16, contents: &str) -> String {
    let (code, desc) = match code {
        200 => (200, "OK"),
        400 => (400, "Bad Request"),
        _ => (500, "Internal Server Error")
    };
    let status_line = format!("HTTP/1.1 {code} {desc}");
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

pub fn parse_request<R: Sized + Read>(buf_reader: BufReader<R>) -> Option<u64> {
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let http_path: Regex = Regex::new(r"^GET /([0-9]+)/? ").unwrap();
    match http_path.captures(&request_line) {
        None => None,
        Some(path) => path[1].parse().ok()
    }
}
