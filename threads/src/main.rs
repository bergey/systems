use std::thread;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};

const FILENAME: &str = "access_counts";

fn main() -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(FILENAME)?;

    let other_thread = thread::spawn(|| {
        // this one does not create nor truncate
        let mut file = OpenOptions::new().write(true).open(FILENAME)?;
        let doc = [1; 1024];
        file.write_all(&doc)?;
        Ok::<(), std::io::Error>(())
    });

    file.seek(SeekFrom::Start(1024))?;
    let doc = [2; 1024];
    file.write_all(&doc)?;

    match other_thread.join() {
        Err(_) => println!("other thread panic"),
        Ok(Ok(())) => (),
        Ok(_) => println!("other thread Error")
    }
    Ok(())
}
