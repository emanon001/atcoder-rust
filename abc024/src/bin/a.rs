use proconio::input;

fn main() {
  input! {
    a: usize, b: usize, c: usize, k: usize,
    s: usize, t: usize
  };

  let res = a * s + b * t - (if s + t >= k { (s + t) * c } else { 0 });
  println!("{}", res);
}
