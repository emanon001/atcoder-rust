use proconio::input;

fn main() {
  input! {
    a: u64, b: u64, k: usize
  };

  let mut a = a;
  let mut b = b;
  let mut t = true;
  for _ in 0..k {
    if t {
      if a % 2 == 1 {
        a -= 1;
      }
      a /= 2;
      b += a;
    } else {
      if b % 2 == 1 {
        b -= 1;
      }
      b /= 2;
      a += b;
    }
    t = !t;
  }
  println!("{} {}", a, b);
}
