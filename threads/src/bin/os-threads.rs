use threads::*;

use std::fs::OpenOptions;
use std::io;
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), io::Error> {
    let file_mutex = Arc::new(Mutex::new(
        OpenOptions::new().read(true).write(true).create(true).truncate(true).open(FILENAME)?
    ));

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let file_mutex = file_mutex.clone();
        thread::spawn(move || {
            handle_connection(stream, file_mutex);
        });
    }

    Ok(())
}

fn write_document(file_mutex: &FileMutex, doc_id: u64, doc: Doc) -> Result<(), io::Error> {
    let mut file = file_mutex.lock().unwrap();
    file.seek(SeekFrom::Start(DOCSIZE as u64 * doc_id))?;
    file.write_all(doc.as_slice())?;
    Ok::<(), std::io::Error>(())
}

fn read_document(file_mutex: &FileMutex, doc_id: u64) -> Result<Doc, std::io::Error> {
    // we could allocate only one Vec, but a real program would do more work here
    let mut buf = vec![0; DOCSIZE];
    let mut file = file_mutex.lock().unwrap();
    let expected_end_byte = DOCSIZE as u64 * (doc_id + 1);
    if file.metadata().unwrap().len() < expected_end_byte {
        Ok(Doc::fill(0))
    } else {
        file.seek(SeekFrom::Start(DOCSIZE as u64 * doc_id)).unwrap();
        file.read(&mut buf).unwrap();
        Ok(Doc::new(buf))
    }
}

fn handle_connection(mut stream: TcpStream, file_mutex: FileMutex) {
    let buf_reader = BufReader::new(&mut stream);
    let response = match parse_request(buf_reader) {
        None => {
            http_reply(400, "GET /<document ID>")
        },
        Some(doc_id) => {
            let doc = read_document(&file_mutex, doc_id).unwrap();
            let new_value = doc.as_byte() + 1;
            write_document(&file_mutex, doc_id, Doc::fill(new_value)).unwrap();
            http_reply(200, &new_value.to_string())
        }
    };
    stream.write_all(response.as_bytes()).unwrap();
}
