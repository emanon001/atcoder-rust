use proconio::input;

fn main() {
  input! {
    n: usize,
    mut lv: [usize; n * 2]
  };

  lv.sort();
  let mut res = 0;
  for i in 0..n {
    res += std::cmp::min(lv[i * 2], lv[i * 2 + 1]);
  }
  println!("{}", res);
}
