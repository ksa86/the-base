use std::io::{self, Read};

//const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> io::Result<()> {

    /*
    let mut buffer = [0; CHUNK_SIZE];
    let bytes_count = io::stdin().read(&mut buffer).unwrap();
     */

    let mut input = io::stdin().lock();
    
    let bytes_count = io::copy(&mut input, &mut io::sink())?;

    println!("bytes_count: {}", bytes_count);

    Ok(())
}
