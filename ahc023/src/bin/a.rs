#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

type Pos = (usize, usize);

/**
 * 区画
 */
struct Grid {
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
     * 区画に作物を植えているか
     */
    planted: Vec<Vec<bool>>,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

impl Grid {
    fn can_move(&self, pos: Pos, dir: Direction) -> bool {
        if !self.planted_at_grid(&pos) {
            return false;
        }
        let new_pos: (isize, isize) = match dir {
            Direction::Left => (pos.0 as isize, pos.1 as isize - 1),
            Direction::Top => (pos.0 as isize - 1, pos.1 as isize),
            Direction::Right => (pos.0 as isize, pos.1 as isize + 1),
            Direction::Bottom => (pos.0 as isize + 1, pos.1 as isize),
        };
        if new_pos.0 < 0
            || new_pos.0 >= self.h as isize
            || new_pos.1 < 0
            || new_pos.1 >= self.w as isize
        {
            return false;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if !self.planted_at_grid(&new_pos) {
            return false;
        }

        // 水路を通らないか
        self.exists_waterway(&pos, dir)
    }

    fn exists_waterway(&self, pos: &Pos, dir: Direction) -> bool {
        match dir {
            Direction::Left => pos.1 >= 1 && self.v_waterway[pos.0][pos.1 - 1] != '#',
            Direction::Top => pos.0 >= 1 && self.h_waterway[pos.0 - 1][pos.1] != '#',
            Direction::Right => pos.1 <= self.w - 2 && self.h_waterway[pos.0][pos.1] != '#',
            Direction::Bottom => pos.0 <= self.h - 2 && self.h_waterway[pos.0][pos.1] != '#',
        }
    }

    fn planted_at_grid(&self, p: &Pos) -> bool {
        self.planted[p.0][p.1]
    }
}

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
