use proconio::input;

fn main() {
  input! {
    n: usize,
    a_l: [i64; n],
    b_l: [i64; n],
  };

  let a_sum: i64 = a_l.iter().sum();
  let b_sum: i64 = b_l.iter().sum();
  if a_sum < b_sum {
    println!("-1");
    std::process::exit(0);
  }

  let mut minus = 0;
  let mut l = Vec::new();
  for i in 0..n {
    let a = a_l[i];
    let b = b_l[i];
    if a >= b {
      l.push(a - b);
    } else {
      minus += b - a;
    }
  }
  l.sort_by_key(|&x| -x);
  let mut res = n - l.len();
  for x in l {
    if minus <= 0 {
      break;
    }
    res += 1;
    minus -= x;
  }
  println!("{}", res);
}
