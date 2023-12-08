use threads::*;

use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};
use tokio::fs::{File, OpenOptions};
use tokio::sync::Mutex;

type FileMutex = Arc<Mutex<File>>;

#[tokio::main]
async fn main() {
    let file_mutex = Arc::new(Mutex::new(
        OpenOptions::new().read(true).write(true).create(true).truncate(true).open(FILENAME)?
    ));

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        let file_mutex = file_mutex.clone();
        tokio::spawn(async move {
            handle_connection(socket, file_mutex).await;
        })
    }
}

fn write_document(file_mutex: &FileMutex, doc_id: u64, doc: Doc) {
    let mut file = file_mutex.lock().await?;
    file.seek(SeekFrom::Start(DOCSIZE as u64 * doc_id)).await;
    file.write_all(doc.as_slice()).await
}

fn read_document(file_mutex: &FileMutex, doc_id: u64) -> Doc {
    let mut buf = vec![0; DOCSIZE];
    let mut file = file_mutex.lock().await.unwrap();
    let expected_end_byte = DOCSIZE as u64 * (doc_id + 1);
    if file.metadata().unwrap().len() < expected_end_byte {
        Ok(Doc::fill(0))
    } else {
        file.seek(SeekFrom::Start(DOCSIZE as u64 * doc_id)).await.unwrap();
        file.read(&mut buf).unwrap();
        Ok(Doc::new(buf))
    }
}

async fn handle_connection(stream: TcpStream, file_mutex: FileMutex) {
    // should we use BufReader?
    let response = match parse_request(stream) {
        None => reply_400(),
        Some(doc_id) => {
            let doc = read_document(&file_mutex, doc_id).unwrap();
            let new_value = doc.as_byte() + 1;
            write_document(&file_mutex, doc_id, Doc::fill(new_value)).unwrap();
            http_reply(200, &new_value.to_string())
        }
    };
    stream.write_all(response.as_bytes()).unwrap();
}
