use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _: usize, a: usize, b: usize,
    s: Chars
  };

  let mut b_count = 0;
  let mut q_count = 0;
  for ch in s {
    match ch {
      'a' => {
        if q_count < a + b {
          q_count += 1;
          println!("Yes");
        } else {
          println!("No");
        }
      }
      'b' => {
        if q_count < a + b && b_count < b {
          b_count += 1;
          q_count += 1;
          println!("Yes");
        } else {
          println!("No");
        }
      }
      'c' => {
        println!("No");
      }
      _ => {}
    }
  }
}
