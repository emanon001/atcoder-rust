use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    a: Chars,
    b: Chars,
    c: Chars
  };

  let res = if a[a.len() - 1] == b[0] && b[b.len() - 1] == c[0] {
    "YES"
  } else {
    "NO"
  };
  println!("{}", res);
}
