use proconio::input;

fn main() {
  input! {
    a: usize, b: usize
  };

  let res = (a - 1) * (b - 1);
  println!("{}", res);
}
