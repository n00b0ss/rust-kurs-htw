

fn main() {

    let list: [i64; 8] = [-2, -2, -1, 0, 1, 2, 3, 4];

    for i in 0..8{
        let y = list[i];
        let result = reciprocal(y);
        print!("Das Reziproke von {} ist ", i);

        match reciprocal(y){
            Some(r) => println!("{r}"),
            None    => println!("n.D")
        }
    }
}
fn reciprocal(n: i64) -> Option<f64> {
    match n {
        0 => None,
        _ =>Some(1.0 /n as f64)
    }
}

