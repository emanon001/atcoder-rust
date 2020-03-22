use proconio::input;

fn main() {
  input! {
    x: i64, a: i64, b: i64
  };

  let res = if a >= b {
    "delicious"
  } else if b - a <= x {
    "safe"
  } else {
    "dangerous"
  };
  println!("{}", res);
}
