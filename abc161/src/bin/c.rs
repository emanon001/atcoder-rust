use proconio::input;

fn main() {
  input! {
    n: u64, k: u64
  };

  if n == 0 {
    println!("0");
  } else if n % k == 0 {
    println!("0");
  } else {
    let mut res = n % k;
    if k - res < res {
      res = k - res;
    }
    println!("{}", res);
  }
}
