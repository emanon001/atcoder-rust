use num::*;
use proconio::input;

fn count(n: u64, c: u64, d: u64) -> u64 {
    let cc = n / c;
    let dc = n / d;
    let lcmc = n / c.lcm(&d);
    n - (cc + dc - lcmc)
}

fn main() {
    input! {
        a: u64, b: u64, c: u64, d: u64
    };

    let res = count(b, c, d) - count(a - 1, c, d);
    println!("{}", res);
}
