use proconio::input;

fn main() {
  input! {
    _: usize, d: usize, k: usize,
    lrv: [(usize, usize); d],
    stv: [(usize, usize); k]
  };

  let mut days = 0;
  let mut res = vec![-1; k];
  let goal = stv.iter().copied().map(|(_, t)| t).collect::<Vec<_>>();
  let mut pos = stv.iter().copied().map(|(s, _)| s).collect::<Vec<_>>();
  for (l, r) in lrv {
    days += 1;
    let mut ok_count = 0;
    for i in 0..k {
      let p = pos[i];
      let g = goal[i];
      let new_p = if p >= l && p <= r {
        if g >= l && g <= r {
          g
        } else if p < g {
          r
        } else {
          l
        }
      } else {
        p
      };
      pos[i] = new_p;
      if pos[i] == g {
        ok_count += 1;
        if res[i] == -1 {
          res[i] = days;
        }
      }
    }
    if ok_count == k {
      for c in res {
        println!("{}", c);
      }
      std::process::exit(0);
    }
  }
}
