use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    i: usize
  };

  println!("{}", s[i - 1]);
}
