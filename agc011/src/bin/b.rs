use proconio::input;

fn main() {
  input! {
    n: usize,
    mut av: [usize; n]
  };

  av.sort();
  let mut minus = 0;
  let mut cur = av[0];
  for (i, a) in av.into_iter().skip(1).enumerate() {
    let i = i + 1;
    if cur * 2 < a {
      minus = i;
    }
    cur += a;
  }
  let res = n - minus;
  println!("{}", res);
}
