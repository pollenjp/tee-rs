use ::std::env;
use ::std::fs::File;
use ::std::io::{self, Read, Write};
use ::std::os::unix::io::AsRawFd;
use ::std::process;

fn open_output(path: &str) -> io::Result<Box<dyn Write>> {
    if path == "-" {
        Ok(Box::new(io::stdout()))
    } else {
        Ok(Box::new(File::create(path)?))
    }
}

fn tee(outputs: &mut [Box<dyn Write>]) -> io::Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut pollfd = libc::pollfd {
        fd: stdin_fd,
        events: libc::POLLIN,
        revents: 0,
    };

    let mut buffer = Vec::new();
    loop {
        // iopoll で入力を待つ
        unsafe {
            libc::poll(&mut pollfd, 1, -1);
        }

        if pollfd.revents & libc::POLLIN != 0 {
            // 入力がある場合
            let mut buf = [0; 4096];
            let bytes_read = io::stdin().read(&mut buf)?;
            if bytes_read == 0 {
                break; // EOF
            }
            buffer.extend_from_slice(&buf[..bytes_read]);

            // 出力
            let s = String::from_utf8_lossy(&buffer);
            for output in outputs.iter_mut() {
                output.write_all(s.as_bytes())?;
                output.flush()?;
            }
            buffer.clear();
        } else if pollfd.revents & libc::POLLHUP != 0 {
            break; // 入力ストリームが閉じられた
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut outputs: Vec<Box<dyn Write>> = vec![Box::new(io::stdout())];
    for path in &args[1..] {
        match open_output(path) {
            Ok(output) => outputs.push(output),
            Err(e) => {
                eprintln!("Error opening {}: {}", path, e);
                process::exit(1);
            }
        }
    }

    if let Err(e) = tee(&mut outputs) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
