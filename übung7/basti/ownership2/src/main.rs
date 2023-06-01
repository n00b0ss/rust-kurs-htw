fn main() {
    let mut a:i32 = 10;
    let mut b:i32 = 15;
    println!("a = {} and b = {}", a, b);

    swap(&mut a, &mut b);

    println!("a = {} and b = {}", a, b);

    let mut s1:String = String::from("Hello");
    let mut s2:String = String::from("World");
    println!("s1 = {} and s2 = {}", s1, s2);

    swap_string(&mut s1, &mut s2);

    println!("s1 = {} and s2 = {}", s1, s2);
}

fn swap(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

fn swap_string(s1: &mut String, s2: &mut String){
    let s3 = s1.clone();
    *s1 = s2.clone();
    *s2 = s3.clone();
}
