use proconio::input;

fn main() {
  input! {
    n: usize,
    ab_l: [(usize, usize); n]
  };

  let mut ab_l = ab_l;
  ab_l.sort_by_key(|&(_, b)| b);
  let mut sum = 0;
  for (a, b) in ab_l {
    sum += a;
    if b < sum {
      println!("No");
      std::process::exit(0);
    }
  }
  println!("Yes");
}
