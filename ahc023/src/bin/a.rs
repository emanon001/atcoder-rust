#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

struct Solver {
    /**
     * 栽培の最大期間
     */
    t: usize,
    /**
     * 区画の縦幅
     */
    h: usize,
    /**
     * 区画の横幅
     */
    w: usize,
    /**
     * 出入口の縦方向の位置
     */
    i0: usize,
    /**
     * 横方向の水路の位置
     */
    h_waterway: Vec<Vec<char>>,
    /**
     * 縦方向の水路の位置
     */
    v_waterway: Vec<Vec<char>>,
    /**
     * 作物の種類
     */
    k: usize,
    /**
     * 作物の栽培プラン
     * (〜ヶ月目までに植える必要がある, 〜ヶ月目に収穫する必要がある)
     */
    plans: Vec<(usize, usize)>,
}

struct Input {
    t: usize,
    h: usize,
    w: usize,
    i0: usize,
    h_waterway: Vec<Vec<char>>,
    v_waterway: Vec<Vec<char>>,
    k: usize,
    plans: Vec<(usize, usize)>,
}

struct Plan {
    k: usize,
    i: usize,
    j: usize,
    s: usize,
}
struct Output {
    m: usize,
    plans: Vec<Plan>,
}

impl Solver {
    fn new(input: Input) -> Self {
        Self {
            t: input.t,
            h: input.h,
            w: input.w,
            i0: input.i0,
            h_waterway: input.h_waterway,
            v_waterway: input.v_waterway,
            k: input.k,
            plans: input.plans,
        }
    }
    fn solve(self) -> Output {
        Output {
            m: 0,
            plans: vec![],
        }
    }
}

fn main() {
    input! {
        t: usize, h: usize, w: usize, i0: usize,
        h_waterway: [Chars; h - 1],
        v_waterway: [Chars; h],
        k: usize,
        plans: [(usize, usize); k]
    };

    let input = Input {
        t,
        h,
        w,
        i0,
        h_waterway,
        v_waterway,
        k,
        plans,
    };
    let solver = Solver::new(input);
    let output = solver.solve();
    println!("{}", output.m);
    println!(
        "{}",
        output
            .plans
            .iter()
            .map(|p| format!("{} {} {} {}", p.k, p.i, p.j, p.s))
            .join("\n")
    );
}
