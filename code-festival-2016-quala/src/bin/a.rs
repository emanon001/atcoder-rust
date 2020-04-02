use proconio::input;

fn main() {
  input! {
    s: String
  };

  let (a, b) = s.split_at(4);
  println!("{} {}", a, b);
}
