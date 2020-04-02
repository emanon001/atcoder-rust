use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars
  };

  let expected: Vec<char> = "CODEFESTIVAL2016".chars().collect();
  let mut res = 0;
  for i in 0..s.len() {
    if s[i] != expected[i] {
      res += 1;
    }
  }
  println!("{}", res);
}
