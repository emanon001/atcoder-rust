use proconio::input;

fn main() {
  input! {
    n: usize,
    abl: [(usize, usize); n]
  };

  let mut abl = abl;
  abl.sort_by_key(|&(a, _)| a);
  let (a, b) = abl.last().unwrap();
  let res = a + b;
  println!("{}", res);
}
