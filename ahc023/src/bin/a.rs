#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Duration, Instant};

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

type BlockWithCost = (Block, usize);

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
    planted_map: HashMap<Block, Crop>,
    crop_to_planted_block: HashMap<usize, Block>,

    far_blocks_with_cost: VecDeque<BlockWithCost>,
    block_to_distance: HashMap<Block, usize>,
    plantable_blocks: BTreeSet<(usize, Block)>,
}

struct CalcAroundBlocksReachableAtHarvest {
    reachable_blocks: Vec<Block>,
    unreachable_blocks: Vec<Block>,
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
        let ground = Self {
            h,
            w,
            h_waterway,
            v_waterway,
            start: (i0, 0),
            planted_map: HashMap::new(),
            crop_to_planted_block: HashMap::new(),
            far_blocks_with_cost: VecDeque::new(),
            block_to_distance: HashMap::new(),
            plantable_blocks: BTreeSet::new(),
        };
        ground.init()
    }

    fn init(mut self) -> Self {
        self.far_blocks_with_cost = self.calculate_far_blocks();
        let mut block_to_distance = HashMap::new();
        let mut distance_to_blocks = BTreeMap::new();
        for &(b, d) in &self.far_blocks_with_cost {
            block_to_distance.insert(b, d);
            distance_to_blocks
                .entry(d)
                .or_insert(BTreeSet::new())
                .insert(b);
        }
        self.block_to_distance = block_to_distance;
        let mut plantable_blocks = BTreeSet::new();
        for (k, v) in distance_to_blocks {
            for v2 in v {
                plantable_blocks.insert((k, v2));
            }
        }
        self.plantable_blocks = plantable_blocks;
        self
    }

    fn plant(&mut self, block: Block, crop: Crop) {
        if self.planted_at_grid(&block) {
            panic!("already planted ({:?})", block);
        }
        self.planted_map.insert(block, crop);
        self.crop_to_planted_block.insert(crop.0, block);
        let distance = self.block_to_distance[&block];
        self.plantable_blocks.remove(&(distance, block));
    }

    fn harvest(&mut self, crop: Crop) -> Option<Block> {
        if let Some(b) = self.crop_to_planted_block.remove(&crop.0) {
            self.planted_map.remove(&b);
            let distance = self.block_to_distance[&b];
            self.plantable_blocks.insert((distance, b));
            for b in self
                .calculate_around_blocks_reachable_at_harvest(b, 0)
                .reachable_blocks
            {
                let distance = self.block_to_distance[&b];
                self.plantable_blocks.insert((distance, b));
            }
            return Some(b);
        }
        None
    }

    fn calculate_far_blocks(&self) -> VecDeque<BlockWithCost> {
        if self.planted_at_grid(&self.start) {
            return VecDeque::new();
        }
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.w]; self.h];
        visited[self.start.0][self.start.1] = true;
        let mut que = VecDeque::new();
        que.push_back((self.start, 1));

        let mut far_blocks: VecDeque<BlockWithCost> = VecDeque::new();
        let dirs = Direction::all();
        while let Some((block, cost)) = que.pop_front() {
            far_blocks.push_front((block, cost));
            for d in &dirs {
                if let Some(new_block) = self.move_block(&block, d, false) {
                    if visited[new_block.0][new_block.1] {
                        continue;
                    }
                    visited[new_block.0][new_block.1] = true;
                    que.push_back((new_block, cost + 1));
                }
            }
        }
        far_blocks
    }

    fn reachable_blocks(&self) -> BTreeSet<Block> {
        if self.planted_at_grid(&self.start) {
            return BTreeSet::new();
        }
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.w]; self.h];
        visited[self.start.0][self.start.1] = true;
        let mut que = VecDeque::new();
        que.push_back(self.start);

        let dirs = Direction::all();
        let mut result = BTreeSet::new();
        while let Some(block) = que.pop_front() {
            result.insert(block);
            for d in &dirs {
                if let Some(new_block) = self.move_block(&block, d, true) {
                    if visited[new_block.0][new_block.1] {
                        continue;
                    }
                    visited[new_block.0][new_block.1] = true;
                    que.push_back(new_block);
                }
            }
        }
        result
    }

    fn calculate_around_blocks_reachable_at_harvest(
        &self,
        fill_block: Block,
        fill_d: usize,
    ) -> CalcAroundBlocksReachableAtHarvest {
        let mut unreachable_blocks = Vec::new();
        let mut reachable_blocks = Vec::new();
        let dirs = Direction::all();
        // 上下左右の区画から見た時に収穫が可能か確認する
        for d in &dirs {
            if let Some(around_block) = self.move_block(&fill_block, d, false) {
                let around_crop = self.planted_map.get(&around_block).unwrap_or(&(0, (0, 0)));

                let mut visited: Vec<Vec<bool>> = vec![vec![false; self.w]; self.h];
                visited[around_block.0][around_block.1] = true;
                let mut que = VecDeque::new();
                que.push_back(around_block);
                let dirs = Direction::all();
                let mut blocks = BTreeSet::new();
                while let Some(block) = que.pop_front() {
                    blocks.insert(block);
                    for d in &dirs {
                        if let Some(new_block) = self.move_block(&block, d, false) {
                            if visited[new_block.0][new_block.1] {
                                continue;
                            }
                            if new_block == fill_block && around_crop.1 .1 < fill_d {
                                continue;
                            }
                            if let Some((_, (_, d))) = self.planted_map.get(&new_block) {
                                if around_crop.1 .1 < *d {
                                    continue;
                                }
                            }
                            visited[new_block.0][new_block.1] = true;
                            que.push_back(new_block);
                        }
                    }
                }
                if blocks.contains(&self.start) {
                    reachable_blocks.push(around_block);
                } else {
                    unreachable_blocks.push(around_block);
                }
            }
        }
        CalcAroundBlocksReachableAtHarvest {
            unreachable_blocks,
            reachable_blocks,
        }
    }

    /**
     * 指定した区画から移動する
     */
    fn move_block(&self, block: &Block, dir: &Direction, check_planted: bool) -> Option<Block> {
        if check_planted && self.planted_at_grid(block) {
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
        if check_planted && self.planted_at_grid(&new_block) {
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

struct OutputPlan {
    k: usize,
    i: usize,
    j: usize,
    s: usize,
}
struct CalculateResult {
    plans: Vec<OutputPlan>,
    score: u64,
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
    /**
     * 乱数生成器
     */
    rng: ThreadRng,
    start: Instant,
}

type Plan = (usize, usize);
type Crop = (usize, Plan);
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
            rng: thread_rng(),
            start: Instant::now(),
        }
    }
    fn solve(&mut self) -> CalculateResult {
        let mut ground = Ground::new(
            self.h,
            self.w,
            self.i0,
            self.h_waterway.clone(),
            self.v_waterway.clone(),
        );

        let sorted = self
            .plans
            .iter()
            .copied()
            .enumerate()
            .map(|(k, p)| (k + 1, p))
            .sorted_by_key(|(_, p)| (p.1, p.0))
            .collect::<Vec<_>>();
        let cur_output = self.simulate(&sorted, &mut ground);

        eprintln!("time: {:?}", Instant::now() - self.start);
        cur_output
    }

    fn simulate(&mut self, input_plans: &Vec<Crop>, ground: &mut Ground) -> CalculateResult {
        let mut s_to_plans = HashMap::new();
        let mut d_to_plans = HashMap::new();
        for (k, (s, d)) in input_plans {
            s_to_plans
                .entry(s)
                .or_insert(Vec::new())
                .push((*k, (*s, *d)));
            d_to_plans
                .entry(d)
                .or_insert(Vec::new())
                .push((*k, (*s, *d)));
        }

        let mut score = 0_u64;
        let mut output_plans = Vec::new();
        for month in 1..=100 {
            if Instant::now() - self.start >= Duration::from_millis(1900) {
                break;
            }
            let mut unreachable_blocks = Vec::new();
            for &(k, (s, d)) in s_to_plans.get(&month).unwrap_or(&Vec::new()) {
                let reachable_blocks = ground.reachable_blocks();
                for (_, b) in ground.plantable_blocks.clone().into_iter().rev() {
                    if !reachable_blocks.contains(&b) {
                        continue;
                    }

                    let check_result = ground.calculate_around_blocks_reachable_at_harvest(b, d);
                    // TLEになる
                    // for b in check_result.reachable_blocks {
                    //     let distance = ground.block_to_distance[&b];
                    //     ground.plantable_blocks.insert((distance, b));
                    // }
                    let mut ok = true;
                    if !check_result.unreachable_blocks.is_empty() {
                        ok = false;
                        unreachable_blocks.push(b.clone());
                    }
                    if !ok {
                        continue;
                    }
                    output_plans.push(OutputPlan {
                        k,
                        i: b.0,
                        j: b.1,
                        s,
                    });
                    ground.plant(b, (k, (s, d)));
                    break;
                }
                for b in &unreachable_blocks {
                    let key = (ground.block_to_distance[b], b.clone());
                    ground.plantable_blocks.remove(&key);
                }
            }
            for &(k, (s, d)) in d_to_plans.get(&month).unwrap_or(&Vec::new()) {
                if let Some(_) = ground.harvest((k, (s, d))) {
                    score += (d - s + 1) as u64;
                }
            }
        }

        CalculateResult {
            plans: output_plans,
            score,
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
    let mut solver = Solver::new(input);
    let output = solver.solve();
    println!("{}", output.plans.len());
    println!(
        "{}",
        output
            .plans
            .iter()
            .map(|p| format!("{} {} {} {}", p.k, p.i, p.j, p.s))
            .join("\n")
    );
}
