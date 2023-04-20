fn main(){
    let fib = fibonacci(32);
    println!("The fibonacci is {}", fib);
}

fn fibonacci(n:i64) -> i64 {

    if n == 1 || n == 0{
       return 1;
    }
        else{
    return fibonacci(n-1)+(n-2);
    }
}