use std::io;
use std::fmt;

fn is_string_integer(s: &str) -> bool {
    match s.trim().parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub struct KomplZahl {
    real: i32,
    img: i32,
}

impl fmt::Display for KomplZahl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}i)", self.real, self.img)
    }
}

pub mod arith {
    use super::KomplZahl;

    pub fn add(z1: &KomplZahl, z2: &KomplZahl) -> KomplZahl {
        let z = KomplZahl {
            real: z1.real + z2.real,
            img: z1.img + z2.img,
        };
        z
    }

    pub fn mult(z1: &KomplZahl, z2: &KomplZahl) -> KomplZahl {
        let z = KomplZahl {
            real: z1.real * z2.real - z1.img * z2.img,
            img: z1.real * z2.img + z2.real * z1.img,
        };
        z
    }
}

mod misc {
    use super::KomplZahl;
    use super::arith;

    pub fn display(komplexe_zahl1: &KomplZahl, komplexe_zahl2: &KomplZahl) {
        let komplexe_zahl3 = arith::add(komplexe_zahl1, komplexe_zahl2);
        let komplexe_zahl4 = arith::mult(komplexe_zahl1, komplexe_zahl2);

        println!("Die erste komplexe Zahl lautet: {}", komplexe_zahl1);
        println!("Die zweite komplexe Zahl lautet: {}", komplexe_zahl2);
        println!("Die addierte komplexe Zahl lautet: {}", komplexe_zahl3);
        println!("Die multiplizierte komplexe Zahl lautet: {}", komplexe_zahl4);
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

    let komplexe_zahl1 = KomplZahl { real: real1, img: img1 };

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

    let komplexe_zahl2 = KomplZahl { real: real2, img: img2 };

    misc::display(&komplexe_zahl1, &komplexe_zahl2);
}
