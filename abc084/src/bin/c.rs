use proconio::input;

fn main() {
  input! {
    n: usize,
    csfv: [(usize, usize, usize); n - 1]
  };

  for i in 0..n - 1 {
    let mut cur = 0_usize;
    for &(c, s, f) in &csfv[i..] {
      if cur <= s {
        cur = s + c;
      } else {
        let m = (cur - s) % f;
        if m == 0 {
          cur += c;
        } else {
          cur += f - m;
          cur += c;
        }
      }
    }
    println!("{}", cur);
  }
  println!("0");
}
