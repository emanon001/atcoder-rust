use proconio::input;

fn main() {
  input! {
    n: usize, t: usize,
    av: [usize; n]
  };

  let mut res = 0;
  let mut cur = av[0];
  for a in av
    .into_iter()
    .skip(1)
    .chain(vec![1_000_000 * 2].into_iter())
  {
    res += std::cmp::min(a - cur, t);
    cur = a;
  }
  println!("{}", res);
}
