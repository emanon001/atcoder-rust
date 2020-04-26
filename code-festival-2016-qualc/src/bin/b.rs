use proconio::input;

fn main() {
  input! {
    k: usize, t: usize,
    mut av: [usize; t]
  };

  if t == 1 {
    println!("{}", k - 1);
    std::process::exit(0);
  }
  av.sort();
  while av.len() > 1 {
    let x = av.pop().unwrap() - 1;
    let y = av.pop().unwrap() - 1;
    if x > 0 {
      av.push(x);
    }
    if y > 0 {
      av.push(y);
    }
    av.sort();
  }
  let res = if av.is_empty() { 0 } else { av[0] - 1 };
  println!("{}", res);
}
