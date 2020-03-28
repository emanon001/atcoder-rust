use proconio::input;

fn main() {
  input! {
    a: isize, op: char, b: isize
  };
  let res = if op == '+' { a + b } else { a - b };
  println!("{}", res);
}
