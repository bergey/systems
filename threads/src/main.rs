use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use regex::Regex;

const FILENAME: &str = "access_counts";
const DOCSIZE: usize = 16;

fn main() -> Result<(), std::io::Error> {
    let file_mutex = Arc::new(Mutex::new(
        OpenOptions::new().write(true).create(true).truncate(true).open(FILENAME)?
    ));

    let threads = (1..10).map(|thread_number| {
        let file_mutex = file_mutex.clone();
        thread::spawn(move || {
            let mut file = file_mutex.lock().unwrap();
            let doc = [thread_number as u8; DOCSIZE];
            file.seek(SeekFrom::Start(DOCSIZE as u64 * thread_number))?;
            file.write_all(&doc)?;
            Ok::<(), std::io::Error>(())
        })
    });

    for other_thread in threads {
        match other_thread.join() {
            Err(_) => println!("other thread panic"),
            Ok(Ok(())) => (),
            Ok(_) => println!("other thread Error")
        }
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let http_path: Regex = Regex::new(r"^GET /([0-9]+)/? ").unwrap();
    let response = match http_path.captures(&request_line) {
        None => {
            println!("Request: {:#?}", request_line);
            http_reply(409, "GET /<document ID>")
        },
        Some(path) => {
            http_reply(200, &path[1])
        }
    };
    stream.write_all(response.as_bytes()).unwrap();
}

fn http_reply(code: u16, contents: &str) -> String {
    let (code, desc) = match code {
        200 => (200, "OK"),
        400 => (400, "Bad Request"),
        _ => (500, "Internal Server Error")
    };
    let status_line = format!("HTTP/1.1 {code} {desc}");
    let length = contents.len();

    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}
