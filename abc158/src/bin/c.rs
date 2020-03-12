use proconio::input;

fn main() {
  input! {
    a: u32,
    b: u32
  };

  for n in 1..=1000 {
    if n * 8 / 100 == a && n * 10 / 100 == b {
      println!("{}", n);
      std::process::exit(0);
    }
  }
  println!("-1");
}
