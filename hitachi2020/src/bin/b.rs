use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    a: usize, b: usize, m: usize,
    al: [usize; a],
    bl: [usize; b],
    xycl: [(Usize1, Usize1, usize); m]
  };

  let mut sorted_al = al.clone();
  sorted_al.sort();
  let mut sorted_bl = bl.clone();
  sorted_bl.sort();

  let mut res = sorted_al[0] + sorted_bl[0];
  for (x, y, c) in xycl {
    let total = al[x] + bl[y] - c;
    if total < res {
      res = total;
    }
  }
  println!("{}", res);
}
