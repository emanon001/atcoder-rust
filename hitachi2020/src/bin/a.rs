use proconio::input;

fn main() {
  input! {
    s: String
  };

  if s.len() % 2 == 1 {
    println!("No");
    std::process::exit(0);
  }

  for i in 0..(s.len() / 2) {
    if &s[(i * 2)..=(i * 2 + 1)] != "hi" {
      println!("No");
      std::process::exit(0);
    }
  }
  println!("Yes");
}
