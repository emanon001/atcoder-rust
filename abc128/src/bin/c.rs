use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize, m: usize,
    ls_list: [[Usize1]; m],
    p_list: [usize; m]
  };

  let mut res = 0;
  for s_state in 0..(1 << n) {
    let is_ok = (0..m).all(|l| {
      let ls = &ls_list[l];
      let mut on_count = 0;
      for s in ls {
        if ((s_state & (1 << s)) >> s) == 1 {
          on_count += 1;
        }
      }
      on_count % 2 == p_list[l]
    });
    if is_ok {
      res += 1;
    }
  }
  println!("{}", res);
}
