use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };
  let res = if s[2] == s[3] && s[4] == s[5] {
    "Yes"
  } else {
    "No"
  };
  println!("{}", res);
}
