use std::array;

mod string_algo;

fn main() {

    println!("{}", string_algo::reverse_string(String::from("The Daily Byte")))
   
}

/*  binary search example
 let items = [1, 4, 5, 9, 11, 16, 20, 35, 89];
    let k = 4;

    match binary_search(k, &items) {
        Some(index) => println!("Found {} at index {}", k, index),
        None => println!("{} not found in the array", k),
    }

    let items = [100, 450, 500, 501, 502, 504, 505, 670, 700, 2000];
    let k = 505;

    match binary_search(k, &items) {
        Some(index) => println!("Found {} at index {}", k, index),
        None => println!("{} not found in the array", k),
    }
 */

fn binary_search(k: i32, items: &[i32]) -> Option<i32> {
    let mut low: i32 = 0;
    let mut high: i32 = items.len() as i32 - 1;

    while low <= high {
        let middle = (low + high) / 2;
        if let Some(current) = items.get(middle as usize) {
            if *current == k {
                return Some(middle);
            }
            if *current > k {
                high = middle - 1
            }
            if *current < k {
                low = middle + 1
            }
        }
    }
    None
}
