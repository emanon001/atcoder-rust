use proconio::input;

fn main() {
  input! {
    l: f64
  };

  let n = l / 3_f64;
  let res = n * n * n;
  println!("{}", res);
}
