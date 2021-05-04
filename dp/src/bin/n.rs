use proconio::input;

fn dfs(l: usize, r: usize, dp: &mut[Vec<Option<(i64, i64)>>], av: &[i64]) -> (i64, i64) {
    if let Some(res) = dp[l][r] {
        return res;
    }
    if l == r {
        return (0, 0);
    }
    if l + 1 == r {
        return (av[l], 0);
    }

    let mut res = (1_i64 << 60, 1_i64 << 60);
    for m in l + 1..r {
        let x = dfs(l, m, dp, av);
        let y = dfs(m, r, dp, av);
        let size = x.0 + y.0;
        let cost = x.1 + y.1 + size;
        if cost < res.1 {
            res = (size, cost);
        }
    }
    // println!("l: {}, r: {}, cost: {}, size: {}", l, r, res.0, res.1);
    dp[l][r] = Some(res);
    res
}

fn main() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let mut dp = vec![vec![None; n + 1]; n + 1];
    let res = dfs(0, n, &mut dp, &av);
    println!("{}", res.1);
}
