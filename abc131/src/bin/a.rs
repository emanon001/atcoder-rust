use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };

    let is_ok = s.windows(2).all(|v| v[0] != v[1]);
    let res = if is_ok { "Good" } else { "Bad" };
    println!("{}", res);
}
