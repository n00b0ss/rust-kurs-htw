fn main() {
    let mut left: i32 = 10;
    let mut right: i32 = 15;
    println!("left = {} and right = {}", left, right);
    gen_swap_2(&mut left, &mut right);
    println!("left = {} and right = {}", left, right);
}




fn gen_swap<T: Clone>(left: &mut T, right: &mut T) {
    let t = left.clone();
    *left = right.clone();
    *right = t;
    }

fn gen_swap_2<T>(a: &mut T, b: &mut T)
    where
        T: Clone,
    {
        (*a, *b) = (b.clone(), a.clone());
    }