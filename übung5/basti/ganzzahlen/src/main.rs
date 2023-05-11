use std::io;

fn is_string_integer(s: &str) -> bool {
    match s.trim().parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn ggT(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        ggT(b, a % b)
    }
}

fn main() {
    let mut string = String::new();
    println!("Bitte geben Sie eine Ganzzahl ein:");
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    println!("Der eingegebene String ist {}", string);

    let number: i32;
    if is_string_integer(&string) {
        number = string.trim().parse::<i32>().unwrap();
        println!("Die eingegebene Zahl ist {}", number);
    } else {
        println!("Error: Der eingegebene String ist keine Ganzzahl.");
        return; // Exit the main function early if input is not an integer
    }

    let mut string2 = String::new();
    println!("Bitte geben Sie eine andere Ganzzahl ein:");
    io::stdin()
        .read_line(&mut string2)
        .expect("Failed to read line");
    println!("Der eingegebene String ist {}", string2);

    let number2: i32;
    if is_string_integer(&string2) {
        number2 = string2.trim().parse::<i32>().unwrap();
        println!("Die eingegebene Zahl ist {}", number2);
    } else {
        println!("Error: Der eingegebene String ist keine Ganzzahl.");
        return; // Exit the main function early if input is not an integer
    }

    let result = ggT(number, number2);
    println!("Der größte gemeinsame Teiler ist {}", result);
}
