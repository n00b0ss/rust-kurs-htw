fn main() {
    let blas: String = String::from("blablablablabla");
    println!("blas = {blas}"); // String
    let blas = blas.len()/3;
    println!("blas = {blas}"); // usize
    let blas = blas > 3;
    println!("blas = {blas}"); // bool
    }