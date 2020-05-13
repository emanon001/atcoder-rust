use proconio::input;

fn main() {
  input! {
    x: usize, y: usize
  };

  let res = if y > x { "Better" } else { "Worse" };
  println!("{}", res);
}
