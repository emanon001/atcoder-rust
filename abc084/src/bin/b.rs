use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    a: usize, _: usize,
    s: Chars
  };

  let is_ok = s[..a].into_iter().all(|&ch| ch.is_digit(10))
    && s[a] == '-'
    && s[a + 1..].into_iter().all(|&ch| ch.is_digit(10));
  let res = if is_ok { "Yes" } else { "No" };
  println!("{}", res);
}
