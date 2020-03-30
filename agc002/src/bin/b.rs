use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize, m: usize,
    xy_l: [(Usize1, Usize1); m]
  };

  let mut l = vec![(false, 1); n];
  l[0] = (true, 1);
  for (x, y) in xy_l {
    match (l[x], l[y]) {
      ((true, 1), (_, c)) => {
        l[x] = (false, 0);
        l[y] = (true, c + 1);
      }
      ((r1, c1), (r2, c2)) => {
        l[x] = (r1, c1 - 1);
        l[y] = (r1 || r2, c2 + 1);
      }
    }
  }
  let res = l.into_iter().filter(|&(r, _)| r).count();
  println!("{}", res);
}
