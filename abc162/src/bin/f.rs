#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;

fn main() {
  input! {
    n: usize,
    av: [i64; n]
  };

  let inf = 1_i64 << 60;
  let m = n / 2;
  let mut dp = vec![std::collections::HashMap::new(); n + 1];
  dp[0].insert(0, vec![0; 3]);
  for i in 0..n {
    let r = (i + 1) / 2;
    let l = if r > 0 { r - 1 } else { 0 };
    for j in l..=r {
      // select
      if j + 1 <= m {
        let mut x = if let Some(x) = dp[i + 1].get(&(j + 1)) {
          x.clone()
        } else {
          vec![-inf; 3]
        };
        x[0] = vec![
          x[0],
          dp[i].get(&j).unwrap()[1] + av[i],
          dp[i].get(&j).unwrap()[2] + av[i],
        ]
        .into_iter()
        .max()
        .unwrap();
        dp[i + 1].insert(j + 1, x);
      }
      // not select
      let mut x = if let Some(x) = dp[i + 1].get(&j) {
        x.clone()
      } else {
        vec![-inf; 3]
      };
      x[1] = vec![x[1], dp[i].get(&j).unwrap()[0]]
        .into_iter()
        .max()
        .unwrap();
      x[2] = vec![x[2], dp[i].get(&j).unwrap()[1]]
        .into_iter()
        .max()
        .unwrap();
      dp[i + 1].insert(j, x);
    }
  }
  println!(
    "{}",
    dp[n].get(&m).unwrap().clone().into_iter().max().unwrap()
  );
}
