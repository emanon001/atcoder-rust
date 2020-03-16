use proconio::input;

fn main() {
  input! {
    n: usize, k: usize,
    a_v: [u64; n]
  };

  let mut res = std::u64::MAX;
  for bits in 0..(1 << n) {
    let mut sum = 0;
    let mut min = 0;
    let mut c = 0;
    for i in 0..n {
      let a = a_v[i];
      if a > min {
        c += 1;
        min = a;
        continue;
      }
      if (bits >> i) & 1 == 1 {
        let new_min = min + 1;
        sum += new_min - a;
        min = new_min;
        c += 1;
      }
    }
    if c >= k && sum < res {
      res = sum;
    }
  }
  println!("{}", res);
}
