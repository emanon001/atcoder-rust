use cargo_snippet::snippet;

#[snippet]
pub fn bit_dp<C>(
    state: usize,
    dp: &mut [Option<i64>],
    candidates: &[C],
    fin: usize,
    inf: i64,
    f: fn(usize, &C) -> (usize, i64),
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
    for c in candidates {
        let (new_state, c) = f(state, c);
        if new_state != state {
            let cost = bit_dp(new_state, dp, candidates, fin, inf, f) + c;
            res = res.min(cost);
        }
    }
    dp[state] = Some(res);
    res
}

#[cfg(test)]
mod tests {
    use super::bit_dp;

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
        let cost = bit_dp(0, &mut dp, &candidates, fin, inf, |state, (s, c)| {
            let mut new_state = state;
            for (i, ch) in s.iter().enumerate() {
                new_state |= if *ch == 'Y' { 1 << i } else { 0 };
            }
            (new_state, *c)
        });
        assert_eq!(cost, 30);
    }
}
