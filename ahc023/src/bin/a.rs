#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

/**
 * 区画
 */
type Block = (usize, usize);

#[derive(Copy, Clone)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![Self::Top, Self::Bottom, Self::Left, Self::Right]
    }
}

/**
 * 区画
 */
struct Ground {
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
    start: Block,
    /**
     * 横方向の水路の位置
     */
    h_waterway: Vec<Vec<char>>,
    /**
     * 縦方向の水路の位置
     */
    v_waterway: Vec<Vec<char>>,
    /**
     * 区画にどの作物を植えているか
     */
    planted_map: HashMap<Block, usize>,
    inverted_planted_map: HashMap<usize, Block>,
}

/**
 * 作物を栽培するための土地
 */
impl Ground {
    fn new(
        h: usize,
        w: usize,
        i0: usize,
        h_waterway: Vec<Vec<char>>,
        v_waterway: Vec<Vec<char>>,
    ) -> Self {
        Self {
            h,
            w,
            h_waterway,
            v_waterway,
            start: (i0, 0),
            planted_map: HashMap::new(),
            inverted_planted_map: HashMap::new(),
        }
    }

    fn plant(&mut self, block: Block, k: usize) {
        if self.planted_at_grid(&block) {
            panic!("already planted ({:?})", block);
        }
        self.planted_map.insert(block, k);
        self.inverted_planted_map.insert(k, block);
    }

    fn harvest_all(&mut self) {
        self.planted_map = HashMap::new();
        self.inverted_planted_map = HashMap::new();
    }

    fn harvest(&mut self, k: usize) -> Option<Block> {
        if let Some(b) = self.inverted_planted_map.remove(&k) {
            self.planted_map.remove(&b);
            return Some(b);
        }
        None
    }

    fn find_far_block(&self) -> Option<Block> {
        if self.planted_at_grid(&self.start) {
            return None;
        }

        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.w]; self.h];
        visited[self.start.0][self.start.1] = true;
        let mut que = VecDeque::new();
        que.push_back(self.start);

        let mut far_block: Option<Block> = None;
        let dirs = Direction::all();
        while let Some(block) = que.pop_front() {
            far_block = Some(block);
            for d in &dirs {
                if let Some(new_block) = self.move_block(&block, d) {
                    if visited[new_block.0][new_block.1] {
                        continue;
                    }
                    visited[new_block.0][new_block.1] = true;
                    que.push_back(new_block);
                }
            }
        }
        far_block
    }

    /**
     * 指定した区画から移動する
     */
    fn move_block(&self, block: &Block, dir: &Direction) -> Option<Block> {
        if self.planted_at_grid(block) {
            return None;
        }

        let new_block: (isize, isize) = match dir {
            Direction::Top => (block.0 as isize - 1, block.1 as isize),
            Direction::Bottom => (block.0 as isize + 1, block.1 as isize),
            Direction::Left => (block.0 as isize, block.1 as isize - 1),
            Direction::Right => (block.0 as isize, block.1 as isize + 1),
        };

        // 土地の範囲外か
        if new_block.0 < 0
            || new_block.0 >= self.h as isize
            || new_block.1 < 0
            || new_block.1 >= self.w as isize
        {
            return None;
        }

        // 作物を植えているか
        let new_block = (new_block.0 as usize, new_block.1 as usize);
        if self.planted_at_grid(&new_block) {
            return None;
        }

        // 水路を通らないか
        if self.exists_waterway(block, dir) {
            None
        } else {
            Some(new_block)
        }
    }

    /**
     * 指定した区画の、指定した方向に水路が存在するか
     */
    fn exists_waterway(&self, block: &Block, dir: &Direction) -> bool {
        match dir {
            Direction::Top => block.0 >= 1 && self.h_waterway[block.0 - 1][block.1] == '1',
            Direction::Bottom => block.0 <= self.h - 2 && self.h_waterway[block.0][block.1] == '1',
            Direction::Left => block.1 >= 1 && self.v_waterway[block.0][block.1 - 1] == '1',
            Direction::Right => block.1 <= self.w - 2 && self.v_waterway[block.0][block.1] == '1',
        }
    }

    /**
     * 指定した区画に作物を植えたか
     */
    fn planted_at_grid(&self, p: &Block) -> bool {
        self.planted_map.contains_key(p)
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
        let mut ground = Ground::new(self.h, self.w, self.i0, self.h_waterway, self.v_waterway);
        // 収穫までが速い, 植えるまでが速い
        let sorted = self
            .plans
            .into_iter()
            .enumerate()
            .sorted_by_key(|(_, p)| (p.1, p.0));
        let head = sorted.clone().take(400);
        let rest = sorted.skip(400).chunks(1200);

        let mut cur_m = 1_usize;
        let mut output_plans = Vec::new();
        {
            let mut next_m = cur_m + 1;
            for (k, (s, d)) in head.sorted_by_key(|(_, p)| (-(p.1 as isize), p.0)) {
                if s < cur_m {
                    continue;
                }
                if let Some((i, j)) = ground.find_far_block() {
                    output_plans.push(Plan {
                        k: k + 1,
                        i,
                        j,
                        s: cur_m,
                    });
                    ground.plant((i, j), k + 1);
                    chmax!(next_m, d + 1);
                }
            }
            cur_m = next_m;
            ground.harvest_all();
        }

        for chunk in &rest {
            let mut next_m = cur_m + 1;
            for (k, (s, d)) in chunk.sorted_by_key(|(_, p)| (-(p.1 as isize), p.0)) {
                if s < cur_m {
                    continue;
                }
                if let Some((i, j)) = ground.find_far_block() {
                    output_plans.push(Plan {
                        k: k + 1,
                        i,
                        j,
                        s: cur_m,
                    });
                    ground.plant((i, j), k + 1);
                    chmax!(next_m, d + 1);
                }
            }
            cur_m = next_m;
            ground.harvest_all();
        }

        Output {
            m: output_plans.len(),
            plans: output_plans,
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
