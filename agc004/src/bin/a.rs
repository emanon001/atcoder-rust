use proconio::input;

fn main() {
  input! {
    a: u64, b: u64, c: u64
  };

  let res = vec![a * c * (b % 2), b * c * (a % 2), a * b * (c % 2)]
    .into_iter()
    .min()
    .unwrap();
  println!("{}", res);
}
