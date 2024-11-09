#![cfg(any(target_os = "linux", target_os = "macos"))]

use ::anyhow::Result;
use ::std::io::{self, Read, Write};
use ::std::os::unix::io::AsRawFd;

pub fn tee(outputs: &mut [Box<dyn Write>]) -> Result<()> {
    let stdin_fd = io::stdin().as_raw_fd();
    let mut pollfd = ::libc::pollfd {
        fd: stdin_fd,
        events: ::libc::POLLIN,
        revents: 0,
    };

    let mut buffer = Vec::new();
    loop {
        // iopoll で入力を待つ
        unsafe {
            ::libc::poll(&mut pollfd, 1, -1);
        }

        if pollfd.revents & ::libc::POLLIN != 0 {
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
        } else if pollfd.revents & ::libc::POLLHUP != 0 {
            break; // 入力ストリームが閉じられた
        }
    }

    Ok(())
}
