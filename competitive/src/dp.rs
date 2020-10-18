use cargo_snippet::snippet;

#[snippet]
pub fn bit_dp<C>(
    state: usize,
    dp: &mut [Option<i64>],
    candidates: &[C],
    fin: usize,
    inf: i64,
    f: fn(usize, &[C], usize) -> (usize, i64),
) -> i64 {
    if let Some(res) = dp[state] {
        return res;
    }
    if state == fin {
        let res = 0;
        dp[state] = Some(res);
        return res;
    }
    let mut res = inf;
    for i in 0..candidates.len() {
        let (new_state, cost) = f(state, candidates, i);
        if new_state != state {
            let cost = bit_dp(new_state, dp, candidates, fin, inf, f) + cost;
            res = res.min(cost);
        }
    }
    dp[state] = Some(res);
    res
}

#[snippet]
pub fn traveling_salesman<C>(
    state: usize,
    u: usize,
    dp: &mut [Vec<Option<i64>>],
    candidates: &[C],
    start: usize,
    fin: usize,
    inf: i64,
    f: fn(usize, &[C], usize, usize) -> (usize, i64),
) -> i64 {
    if let Some(res) = dp[state][u] {
        return res;
    }
    if state == fin && u == start {
        dp[state][u] = Some(0);
        return 0;
    }
    let mut res = inf;
    for v in 0..candidates.len() {
        let (new_state, cost) = f(state, candidates, u, v);
        if new_state != state {
            let cost = traveling_salesman(new_state, v, dp, candidates, start, fin, inf, f) + cost;
            res = res.min(cost);
        }
    }
    dp[state][u] = Some(res);
    res
}

#[cfg(test)]
mod tests {
    use super::{bit_dp, traveling_salesman};

    #[test]
    fn test_bit_dp() {
        // ref: https://atcoder.jp/contests/past201912-open/tasks/past201912_i
        let candidates = vec![
            ("YYY".chars().collect::<Vec<char>>(), 100),
            ("YYN".chars().collect::<Vec<char>>(), 20),
            ("YNY".chars().collect::<Vec<char>>(), 10),
            ("NYY".chars().collect::<Vec<char>>(), 25),
        ];
        let fin = (1 << 3) - 1;
        let inf = 1_i64 << 60;
        let mut dp = vec![None; fin + 1];
        let cost = bit_dp(0, &mut dp, &candidates, fin, inf, |state, candidates, i| {
            let (s, c) = &candidates[i];
            let mut new_state = state;
            for (i, ch) in s.iter().enumerate() {
                new_state |= if *ch == 'Y' { 1 << i } else { 0 };
            }
            (new_state, *c)
        });
        assert_eq!(cost, 30);
    }

    #[test]
    fn test_traveling_salesman() {
        // ref. https://atcoder.jp/contests/abc180/tasks/abc180_e
        let candidates: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (1, 1, 1), (-1, -1, -1)];
        let fin = (1 << 3) - 1;
        let inf = 1_i64 << 60;
        let mut dp = vec![vec![None; 3]; fin + 1];
        let cost = traveling_salesman(
            0,
            0,
            &mut dp,
            &candidates,
            0,
            fin,
            inf,
            |state, candidates, u, v| {
                let (ux, uy, uz) = candidates[u];
                let (vx, vy, vz) = candidates[v];
                let new_state = state | (1 << v);
                let cost = (vx - ux).abs() + (vy - uy).abs() + 0.max(vz - uz);
                (new_state, cost)
            },
        );
        assert_eq!(cost, 10);
    }
}
