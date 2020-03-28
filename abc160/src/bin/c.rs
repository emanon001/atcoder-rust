use proconio::input;

fn main() {
  input! {
    k: usize, n: usize,
    a_l: [isize; n]
  };

  let mut a_l = a_l;
  a_l.sort();
  let mut diff = 0;
  for i in 0..n {
    let n = if i == n - 1 {
      (k as isize) - a_l[i] + a_l[0]
    } else {
      a_l[i + 1] - a_l[i]
    };
    if n > diff {
      diff = n;
    }
  }
  let res = (k as isize) - diff;
  println!("{}", res);
}
