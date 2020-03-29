use proconio::input;

fn main() {
  input! {
    h: usize, w: usize, k: usize
  };

  for a in 0..=h {
    for b in 0..=w {
      let x = a * w + b * h - a * b * 2;
      if x == k {
        println!("Yes");
        std::process::exit(0);
      }
    }
  }
  println!("No");
}
