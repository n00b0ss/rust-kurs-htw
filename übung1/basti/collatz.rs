fn main() {
    let mut max_len = 0;
    let mut max_start = 0;

    for n in 2..1000000 {
        let mut num = n as u64;
        let mut len = 1;

        while num > 1 {
            num = if num % 2 == 0 {
                num / 2
            } else {
                3 * num + 1
            };
            len += 1;
        }

        if len > max_len {
            max_len = len;
            max_start = n;
        }
    }

    println!("Starting number {} produces a sequence of length {}", max_start, max_len);
}
