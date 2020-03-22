use proconio::input;

pub fn rest_square_coordinates(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
  let c = (b.0 - b.1 + a.1, b.1 + b.0 - a.0);
  let d = (c.0 - c.1 + b.1, c.1 + c.0 - b.0);
  (c, d)
}

fn main() {
  input! {
    a: (i32, i32),
    b: (i32, i32),
  };
  let (c, d) = rest_square_coordinates(a, b);
  println!("{} {} {} {}", c.0, c.1, d.0, d.1);
}
