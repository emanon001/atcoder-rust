use proconio::input;

fn main() {
  input! {
    n: u64, a: u64, b: u64
  };

  if a > b {
    println!("0");
    std::process::exit(0);
  }
  if a == b {
    println!("1");
    std::process::exit(0);
  }
  if n < 2 {
    println!("0");
    std::process::exit(0);
  }
  if n == 2 {
    println!("1");
    std::process::exit(0);
  }

  let m = n - 2;
  let c = b - a;
  let res = m * c + 1;
  println!("{}", res);
}
