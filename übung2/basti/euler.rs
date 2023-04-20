use std::thread;
use std::time::Duration;

fn main() {
    let mut e: f64 = 1.0;
    let mut k: f64 = 1.0;
    let mut n: f64 = 1.0;

    loop {
        e = e + (1.0 / k);
        k = k * n;
        n = n+ 1.0;
        println!("{}", e);
        thread::sleep(Duration::from_secs(1));
    }
}
