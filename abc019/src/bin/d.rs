use std::io::*;

fn read() -> String {
  let mut s = String::new();
  stdin().read_line(&mut s).ok();
  s.trim().to_string()
}

fn main() {
  let n = read().parse::<usize>().unwrap();

  let mut longest = (1, 0);
  for v in 2..=n {
    println!("? {} {}", 1, v);
    stdout().flush().ok();
    let d = read().parse::<usize>().unwrap();
    if d > longest.1 {
      longest = (v, d);
    }
  }
  let mut res = 0;
  let u = longest.0;
  for v in 1..=n {
    if u == v {
      continue;
    }
    println!("? {} {}", u, v);
    stdout().flush().ok();
    let d = read().parse::<usize>().unwrap();
    if d > res {
      res = d;
    }
  }
  println!("! {}", res);
}
