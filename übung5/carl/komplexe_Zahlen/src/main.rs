#[derive (Debug)]
struct KomplexeZahl {
    realteil: i32,
    imaginaerteil: i32,
}

fn addieren(a: &KomplexeZahl, b: &KomplexeZahl) -> KomplexeZahl {
    let z = KomplexeZahl{
        realteil: a.realteil + b.realteil,
        imaginaerteil: a.imaginaerteil + b.imaginaerteil,
    };
    z
}

fn subtrahieren(a: &KomplexeZahl, b: &KomplexeZahl) -> KomplexeZahl {
    let z = KomplexeZahl {
        realteil: a.realteil - b.realteil,
        imaginaerteil: a.imaginaerteil - b.imaginaerteil,
    };
    z
}

fn multiplizieren(a: &KomplexeZahl, b: &KomplexeZahl) -> KomplexeZahl {
    let z = KomplexeZahl {
        realteil: a.realteil*b.realteil-a.imaginaerteil*b.imaginaerteil,
        imaginaerteil: a.realteil*b.imaginaerteil+b.realteil*a.imaginaerteil,
    };
    z
}

fn main() {
    let a = KomplexeZahl {
        realteil: 3,
        imaginaerteil: 2,
    };

    let b = KomplexeZahl {
        realteil: 6,
        imaginaerteil: 5,
    };

    //print!("{:#?}+{:#?}i + {:#?}+{:#?}i", a.realteil, a.imaginaerteil, b.realteil, b.imaginaerteil);
    let z = addieren(&a, &b);
    
    println!("{:#?}+{:#?}i + {:#?}+{:#?}i = {:#?}+{:#?}i", a.realteil, a.imaginaerteil, b.realteil, b.imaginaerteil, z.realteil, z.imaginaerteil);

    let z = subtrahieren(&a, &b);

    println!("{:#?}+{:#?}i - {:#?}+{:#?}i = {:#?}+{:#?}i", a.realteil, a.imaginaerteil, b.realteil, b.imaginaerteil, z.realteil, z.imaginaerteil);
    
    let z = multiplizieren(&a, &b);

    println!("{:#?}+{:#?}i * {:#?}+{:#?}i = {:#?}+{:#?}i", a.realteil, a.imaginaerteil, b.realteil, b.imaginaerteil, z.realteil, z.imaginaerteil);

}
