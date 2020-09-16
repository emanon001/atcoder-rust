use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize
    };

    let sum = 45 * 45;
    let rest = sum - n;
    let mut table = HashMap::new();
    for x in 1..=9 {
        for y in 1..=9 {
            let z = x * y;
            table.entry(z).or_insert(Vec::new()).push((x, y));
        }
    }
    for (x, y) in table.get(&rest).unwrap() {
        println!("{} x {}", x, y);
    }
}
