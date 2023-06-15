use std::cmp::Ordering;

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 5;
    let index = gen_bin_search(&numbers, &target);
    println!("Found {} at index {}", target, index.unwrap());

    let strings = ["apple", "banana", "cherry", "date", "fig", "grape"];
    let target_str = "banana";
    let index_str = gen_bin_search(&strings, &target_str);
    println!("Found {} at index {}", target_str, index_str.unwrap());
}

fn gen_bin_search<T: PartialOrd + std::cmp::Ord>(field: &[T], target: &T) -> Option<usize> {
    match field.binary_search(target) {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}
