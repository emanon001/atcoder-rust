use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
      n: usize, m: usize, q: usize,
      lrv: [(Usize1, Usize1); m],
      pqv: [(Usize1, Usize1); q]
    };

    let mut cv = vec![vec![0; n]; n];
    for (l, r) in lrv {
        cv[l][r] += 1;
    }
    let mut cucusum = vec![vec![0; n + 1]; n + 1];
    for l in 0..n {
        for r in 0..n {
            cucusum[l + 1][r + 1] =
                cucusum[l + 1][r] + cucusum[l][r + 1] - cucusum[l][r] + cv[l][r];
        }
    }
    for (p, q) in pqv {
        let res = cucusum[q + 1][q + 1] - cucusum[q + 1][p] - cucusum[p][q + 1] + cucusum[p][p];
        println!("{}", res);
    }
}
