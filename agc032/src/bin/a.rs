use proconio::input;

fn check(l: &[usize]) -> bool {
  for (i, &x) in l.iter().enumerate() {
    let i = i + 1;
    if x > i {
      return false;
    }
  }
  true
}

fn main() {
  input! {
    n: usize,
    b_l: [usize; n]
  };

  let mut b_l = b_l;
  let mut res = Vec::new();
  for _ in 0..n {
    if !check(&b_l) {
      println!("-1");
      std::process::exit(0);
    }
    let (i, &b) = b_l
      .iter()
      .enumerate()
      .rev()
      .find(|&(i, &b)| i + 1 == b)
      .unwrap();
    res.push(b);
    b_l.remove(i);
  }
  for b in res.into_iter().rev() {
    println!("{}", b);
  }
}
