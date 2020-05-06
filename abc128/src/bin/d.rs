use proconio::input;

fn main() {
  input! {
    n: usize, k: usize,
    vv: [isize; n]
  };

  let mut res = -(1_isize << 30);
  for pop_c in 0..=std::cmp::min(k, n) {
    for left_c in 0..=pop_c {
      let mut v = Vec::new();
      for x in &vv[0..left_c] {
        v.push(*x);
      }
      let right_c = pop_c - left_c;
      for x in &vv[n - right_c..] {
        v.push(*x);
      }
      v.sort();
      let mut push_c = k - pop_c;
      let mut sum = 0_isize;
      for x in v {
        if x < 0 && push_c > 0 {
          push_c -= 1;
        } else {
          sum += x;
        }
      }
      if sum > res {
        res = sum;
      }
    }
  }
  println!("{}", res);
}
