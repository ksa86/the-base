use std::io::{self, Read};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> io::Result<()> {
    
    let mut total_bytes = 0;
    let mut total_words = 0;
    let mut total_lines = 0;
    let mut word_inside = 0;

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num = match io::stdin().read(&mut buffer){
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break, // panic!("reading error")
        };

        total_bytes += num;
        
        for val_ref in &buffer[..num] {

            match (val_ref, val_ref.is_ascii_whitespace()) {                
                (b'\n', _) => {
                    total_lines += 1;                        
                },                
                (_, true) => {
                    total_words += word_inside;
                    word_inside = 0;
                },
                _ => {
                    word_inside = 1;
                },
            }            
        }
    }

    println!("{} {} {}", total_lines, total_words, total_bytes);

    Ok(())
}
