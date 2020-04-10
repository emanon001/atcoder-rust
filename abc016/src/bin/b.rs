use proconio::input;

fn main() {
  input! {
    a: isize, b: isize, c: isize
  };
  let is_plus = a + b == c;
  let is_minus = a - b == c;
  let res = match (is_plus, is_minus) {
    (true, true) => '?',
    (true, false) => '+',
    (false, true) => '-',
    _ => '!',
  };
  println!("{}", res);
}
