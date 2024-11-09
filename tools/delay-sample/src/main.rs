use std::thread::sleep;
use std::time::Duration;

fn main() {
    let count = 3;
    println!("最初の出力");

    sleep(Duration::from_secs(count));

    println!("{}秒後の出力", count);
}
