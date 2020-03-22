use proconio::input;

fn main() {
  input! {
    n: usize, k: usize
  };
  if k == 1 {
    println!("0");
    std::process::exit(0);
  }
  let res = n - k;
  println!("{}", res);
}
