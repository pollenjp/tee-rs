#[cfg(any(target_os = "linux", target_os = "macos"))]
pub mod unix;
#[cfg(target_os = "windows")]
pub mod windows;

use ::anyhow::Result;
use ::std::env;
use ::std::fs::File;
use ::std::io::{self, Write};
use ::std::process;

fn open_output(path: &str) -> Result<Box<dyn Write>> {
    if path == "-" {
        Ok(Box::new(io::stdout()))
    } else {
        Ok(Box::new(File::create(path)?))
    }
}

fn tee(outputs: &mut [Box<dyn Write>]) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        unix::tee(outputs)
    }

    #[cfg(target_os = "windows")]
    {
        windows::tee(outputs)
    }
}

pub fn run() -> Result<()> {
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

    tee(&mut outputs)
}
