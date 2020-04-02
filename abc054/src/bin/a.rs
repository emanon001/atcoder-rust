use proconio::input;

fn main() {
  input! {
    a: String, b: String
  };

  let a = i32::from_str_radix(&a, 10).unwrap();
  let b = i32::from_str_radix(&b, 10).unwrap();
  let res = if a == b {
    "Draw"
  } else if a == 1 || (b != 1 && a > b) {
    "Alice"
  } else {
    "Bob"
  };
  println!("{}", res);
}
