use proconio::input;

fn main() {
  input! {
    x: u64, y: u64
  };
  if x % y == 0 {
    println!("-1");
  } else {
    println!("{}", x);
  }
}
