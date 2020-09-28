use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        w: Chars
    };

    let vowels = vec!['a', 'i', 'u', 'e', 'o'];
    let res = w
        .into_iter()
        .filter(|ch| !vowels.contains(ch))
        .collect::<String>();
    println!("{}", res);
}
