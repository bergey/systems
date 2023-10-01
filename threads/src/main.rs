use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};
use std::thread;

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
    Ok(())
}
