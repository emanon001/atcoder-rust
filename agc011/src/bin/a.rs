use proconio::input;

fn main() {
  input! {
    n: usize, c: usize, k: usize,
    t_l: [usize; n]
  };

  let mut t_l = t_l;
  t_l.sort();
  t_l.push(std::usize::MAX);
  let mut res = 0;
  let mut head_t = t_l[0];
  let mut wait_c = 0;
  for t in t_l {
    if head_t + k < t {
      res += 1;
      head_t = t;
      wait_c = 1;
    } else {
      if wait_c == c {
        res += 1;
        head_t = t;
        wait_c = 1;
      } else {
        wait_c += 1;
      }
    }
  }
  println!("{}", res);
}
