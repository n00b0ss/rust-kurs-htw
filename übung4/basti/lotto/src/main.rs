fn main() {
    let tipp : [u8;6] = [1, 5, 13, 19, 29, 32];
    let win = tipp;
    println!("Der Tipp ist = {:?}", tipp);    
    let tipp : [u8;6] = [1, 5, 13, 19, 29, 33];
    println!("Der manipulierte Tipp ist = {:?}", tipp);  
    println!("{}", {
        if tipp == win {
            "Gewonnen!"
        }
        else {
        "Verloren!"
        }

    })
}
