#![cfg(windows)]

use ::anyhow::Result;
use ::std::io::{self, Write};

pub fn tee(outputs: &mut [Box<dyn Write>]) -> Result<()> {
    let input = io::stdin();

    loop {
        let mut buf = String::new();
        let bytes_read = match input.read_line(&mut buf) {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                eprintln!("Error reading from stdin: {}", error);
                break;
            }
        };
        if bytes_read == 0 {
            break;
        }

        // println!("debug: {:#04X?}", buf.as_bytes());

        for output in outputs.iter_mut() {
            match output.write_all(buf.as_bytes()) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Error writing to output: {}", error);
                    break;
                }
            };
            output.flush()?;
        }
    }

    Ok(())
}
