fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut sum;
    loop {
        println!("{}", a);
        sum = a + b;
        a = b;
        b = sum;
    }
}