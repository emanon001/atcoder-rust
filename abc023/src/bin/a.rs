use proconio::input;

fn main() {
  input! {
    x: usize
  };

  let res = x / 10 + x % 10;
  println!("{}", res);
}
