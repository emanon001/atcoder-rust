use proconio::input;

fn main() {
  input! {
    x: i64, y: i64
  };

  if x < y {
    if (x <= 0 && y <= 0) || (x >= 0 && y >= 0) {
      // x: 0, y: 2
      // x: -2, y: 0
      // x: -5, y: -2
      // x: 2, y: 5
      println!("{}", (x - y).abs());
    } else {
      // x: -5, y: 2
      // x: -2, y: 5
      println!("{}", (x.abs() - y.abs()).abs() + 1);
    }
  } else {
    if (x < 0 && y < 0) || (x > 0 && y > 0) {
      // x: -2, y: -5
      // x: 5, y: 2
      println!("{}", (x - y).abs() + 2);
    } else {
      // x: 2, y: 0
      // x: 0, y: -2
      // x: 2, y: -5
      // x: 5, y: -2
      println!("{}", (x.abs() - y.abs()).abs() + 1);
    }
  }
}
