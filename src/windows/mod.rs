#![cfg(windows)]

use ::anyhow::Result;
use ::std::env;
use ::std::fs::File;
use ::std::io::{self, BufRead, Read, Write};
use ::std::process;
// use mio::{Events, Interest, Poll, Token};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn tee(outputs: &mut [Box<dyn Write>]) -> Result<()> {
    let (tx, rx) = channel();

    let stdin = std::io::stdin();

    // スレッド内で標準入力から読み込み
    let handle = thread::spawn(move || {
        let mut stdin = stdin.lock();
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = stdin.read(&mut buffer).expect("Error reading from stdin");
            if bytes_read == 0 {
                break;
            }

            tx.send(buffer[..bytes_read].to_vec()).unwrap();
        }
    });

    let mut output_buffer = Vec::new();
    let timeout = Duration::from_millis(100);
    loop {
        match rx.recv_timeout(timeout) {
            Ok(data) => {
                for output in outputs.iter_mut() {
                    output.write_all(&data)?;
                }
                output_buffer.extend_from_slice(&data);
            }
            Err(_) => {
                if output_buffer.is_empty() {
                    for output in outputs {
                        output.write_all(&output_buffer)?;
                    }
                }
                break;
            }
        }
    }

    print!("");
    handle.join().unwrap();
    Ok(())
}

pub fn tee2(outputs: &mut [Box<dyn Write>]) -> Result<()> {
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

        println!("debug: {:#04X?}", buf.as_bytes());

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

pub fn tee3(outputs: &mut [Box<dyn Write>]) -> Result<()> {
    let mut buffer = Vec::new();

    loop {
        let mut buf = vec![0; 1];
        let bytes_read = match ::std::io::stdin().lock().read_until(b'\r', &mut buf) {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                eprintln!("Error reading from stdin: {}", error);
                break;
            }
        };

        if bytes_read == 0 {
            break;
        }

        println!("debug: {:#04X?}", buf);

        buffer.extend_from_slice(&buf[..bytes_read]);
        let s = String::from_utf8_lossy(&buffer);
        for output in outputs.iter_mut() {
            match output.write_all(s.as_bytes()) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Error writing to output: {}", error);
                    break;
                }
            };
            output.flush()?;
        }
        buffer.clear();
    }

    Ok(())
}
