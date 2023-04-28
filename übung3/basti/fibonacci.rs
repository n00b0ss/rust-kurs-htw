use std::time::Duration;
fn main(){
    let mut n = 0;
    loop{
    let fib = fibonacci(n);
    println!("The fibonacci is {}", fib);
    println!("The counter is {}", n);
    n = n +1;
    std::thread::sleep(Duration::from_secs(1));
    }
}

fn fibonacci(n:i64) -> i64 {

    if n == 1 || n == 0{
       return 1;
    }
        else{
    return fibonacci(n-1)+(n-2);
    }
}
