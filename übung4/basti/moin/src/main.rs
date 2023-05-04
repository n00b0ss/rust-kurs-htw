fn main() {
    
    let mut x = String::from("Moin."); 
    {
        
        println!("inner first: {x}"); 
        let y = hello(); 
                         
        println!("inner then: {y}"); 
    } 
    println!("outer: {x}"); 
} 
fn hello() -> String {
    
    String::from("Hello, world!") 
} 
