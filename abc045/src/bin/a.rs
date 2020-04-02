use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
    h: usize
  };

  let res = (a + b) * h / 2;
  println!("{}", res);
}
