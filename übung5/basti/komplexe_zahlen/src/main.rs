use std::io;
use std::fmt;

fn is_string_integer(s: &str) -> bool {
    match s.trim().parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

struct KomplZahl {
    real: i32,
    img: i32,
}

impl fmt::Display for KomplZahl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}i)", self.real, self.img)
    }
}

impl KomplZahl {
    fn new(real: i32, img: i32) -> KomplZahl {
        KomplZahl { real, img }
    }
    
    fn add(&self, other: &KomplZahl) -> KomplZahl {
        let z = KomplZahl {
            real: self.real + other.real,
            img: self.img + other.img,
        };
        z
    }
    
    fn mult(&self, other: &KomplZahl) -> KomplZahl {
        let z = KomplZahl {
            real: self.real * other.real - self.img * other.img,
            img: self.real * other.img + other.real * self.img,
        };
        z
    }
}

fn main() {
    let mut string = String::new();
    println!("Bitte geben Sie den Realteil für die erste komplexe Zahl ein:");
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    let real1: i32;
    if is_string_integer(&string) {
        real1 = string.trim().parse::<i32>().unwrap();
    } else {
        println!("Error: Der eingegebene String ist kein i32");
        return;
    }

    let mut string2 = String::new();
    println!("Bitte geben Sie den Imaginärteil für die erste komplexe Zahl ein:");
    io::stdin()
        .read_line(&mut string2)
        .expect("Failed to read line");

    let img1: i32;
    if is_string_integer(&string2) {
        img1 = string2.trim().parse::<i32>().unwrap();
    } else {
        println!("Error: Der eingegebene String ist kein i32");
        return;
    }

    let komplexe_zahl1 = KomplZahl::new(real1, img1);

    let mut string3 = String::new();
    println!("Bitte geben Sie den Realteil für die zweite komplexe Zahl ein:");
    io::stdin()
        .read_line(&mut string3)
        .expect("Failed to read line");

    let real2: i32;
    if is_string_integer(&string3) {
        real2 = string3.trim().parse::<i32>().unwrap();
    } else {
        println!("Error: Der eingegebene String ist kein i32");
        return;
    }

    let mut string4 = String::new();
    println!("Bitte geben Sie den Imaginärteil für die zweite komplexe Zahl ein:");
    io::stdin()
        .read_line(&mut string4)
        .expect("Failed to read line");

    let img2: i32;
    if is_string_integer(&string4) {
        img2 = string4.trim().parse::<i32>().unwrap();
    } else {
        println!("Error: Der eingegebene String ist kein i32");
        return;
    }

    let komplexe_zahl2 = KomplZahl::new(real2, img2);
    println!("Die erste komplexe Zahl lautet: {}", komplexe_zahl1);
    println!("Die zweite komplexe Zahl lautet: {}", komplexe_zahl2);
    
    let komplexe_zahl3 = komplexe_zahl1.add(&komplexe_zahl2);
    println!("Die addierte komplexe Zahl lautet: {}", komplexe_zahl3);
    
    let komplexe_zahl4 = komplexe_zahl1.mult(&komplexe_zahl2);
    println!("Die multiplizierte komplexe Zahl lautet: {}", komplexe_zahl4);
}
  
