use proconio::input;

fn main() {
  input! {
    a: usize, p: usize
  };

  let res = (a * 3 + p) / 2;
  println!("{}", res);
}
