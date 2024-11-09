use ::std::process;

fn main() {
    if let Err(e) = tee_rs::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
