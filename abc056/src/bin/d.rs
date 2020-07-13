use proconio::input;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

fn main() {
    input! {
        n: usize, k: usize,
        mut av: [usize; n]
    };

    av.sort();

    let res = bsearch(-1, n as i64, |x| {
        let x = x as usize;
        let mut dp = vec![vec![false; k]; n + 1];
        dp[0][0] = true;
        for i in 0..n {
            for j in 0..k {
                if i == x {
                    dp[i + 1][j] = dp[i][j];
                    continue;
                }

                dp[i + 1][j] |= dp[i][j];
                let a = av[i];
                if j + a >= k {
                    continue;
                }
                dp[i + 1][j + a] |= dp[i][j];
            }
        }
        let a = av[x];
        let l = (k as isize - a as isize).max(0) as usize;
        let is_needed = (l..k).any(|i| dp[n][i]);
        !is_needed
    });
    let res = res.map(|x| x + 1).unwrap_or(0);
    println!("{}", res);
}
