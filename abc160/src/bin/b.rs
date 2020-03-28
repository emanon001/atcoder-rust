use proconio::input;

fn main() {
  input! {
    x: usize
  };

  let a = x / 500;
  let mut res = a * 1000;
  let rest = x % 500;
  res += (rest / 5) * 5;
  println!("{}", res);
}
