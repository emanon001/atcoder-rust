use proconio::input;

fn main() {
  input! {
    a: i64, b: i64
  };
  if a <= 0 && b >= 0 {
    println!("Zero");
  } else if a > 0 {
    println!("Positive");
  } else {
    let minus_count = (a - b) + 1;
    if minus_count % 2 == 0 {
      println!("Positive");
    } else {
      println!("Negative");
    }
  }
}
