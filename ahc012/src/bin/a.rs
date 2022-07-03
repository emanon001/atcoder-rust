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

const K: usize = 100;

type Score = u64;
type Line = i64;

pub fn compress_zahyo<T: Ord + std::hash::Hash + Clone>(
    zahyo: &[T],
) -> std::collections::HashMap<T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x);
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x.clone(), i);
    }
    map
}

struct ScoreCalculator {}

impl ScoreCalculator {
    pub fn calculate(
        av: &Vec<usize>,
        strawberry_cusum: &Vec<Vec<usize>>,
        compressed_zahyo_horizontal: &HashMap<i64, usize>,
        compressed_zahyo_vertical: &HashMap<i64, usize>,
        horizontal_lines: &BTreeSet<Line>,
        vertical_lines: &BTreeSet<Line>,
        debug: bool,
    ) -> Score {
        if horizontal_lines.len() + vertical_lines.len() > K {
            panic!("line count > k");
        }

        // 切り分け
        let edge = 10_i64.pow(9) + 2;
        let mut counts = vec![0_usize; 11];
        for (y1, y2) in vec![-edge]
            .iter()
            .chain(horizontal_lines.iter())
            .chain(vec![edge].iter())
            .tuple_windows()
        {
            let y1 = *compressed_zahyo_horizontal.get(y1).unwrap();
            let y2 = *compressed_zahyo_horizontal.get(y2).unwrap();
            for (x1, x2) in vec![-edge]
                .iter()
                .chain(vertical_lines.iter())
                .chain(vec![edge].iter())
                .tuple_windows()
            {
                let x1 = *compressed_zahyo_vertical.get(x1).unwrap();
                let x2 = *compressed_zahyo_vertical.get(x2).unwrap();
                let c = strawberry_cusum[y2][x2]
                    - strawberry_cusum[y2][x1 + 1]
                    - strawberry_cusum[y1 + 1][x2]
                    + strawberry_cusum[y1 + 1][x1 + 1];
                if c < counts.len() {
                    counts[c] += 1;
                }
            }
        }

        if debug {
            eprintln!("{:?}", counts);
        }

        // スコア計算
        let mut a = 0_usize;
        let mut b = 0_usize;
        for d in 0..10 {
            a += av[d].min(counts[d + 1]);
            b += av[d];
        }
        (10.pow(6) as f64 * (a as f64 / b as f64)).round() as u64
    }
}

struct Scores {
    max_size: usize,
    score_map: BTreeMap<Score, (BTreeSet<Line>, BTreeSet<Line>)>,
}

impl Scores {
    fn new(max_size: usize) -> Self {
        let score_map = BTreeMap::new();
        Self {
            score_map,
            max_size,
        }
    }
    fn update_max_size(&mut self, max_size: usize) {
        assert!(max_size >= self.max_size);
        self.max_size = max_size;
    }

    fn update_if_needed(
        &mut self,
        score: Score,
        horizontal_lines: &BTreeSet<Line>,
        vertical_lines: &BTreeSet<Line>,
    ) {
        let mut remove_key: Option<Score> = None;
        if let Some((s, _)) = self.score_map.iter().next() {
            if self.score_map.len() >= self.max_size {
                if &score < s {
                    return;
                }
                remove_key = Some(s.clone());
            }
        }
        if let Some(key) = remove_key {
            self.score_map.remove(&key);
        }
        self.score_map
            .insert(score, (horizontal_lines.clone(), vertical_lines.clone()));
    }

    fn get_scores(&self) -> Vec<(Score, (BTreeSet<Line>, BTreeSet<Line>))> {
        let mut res = Vec::new();
        for v in self.score_map.iter().rev() {
            res.push((v.0.clone(), v.1.clone()));
        }
        res
    }
}

struct Solver {
    n: usize,
    k: usize,
    av: Vec<usize>,
    strawberry_list: Vec<(i64, i64)>,
    strawberry_cusum: Vec<Vec<usize>>,
    compressed_strawberry_horizontal: HashMap<i64, usize>,
    compressed_strawberry_vertical: HashMap<i64, usize>,
    start_time: Instant,
    rng: ThreadRng,
}

impl Solver {
    fn new(n: usize, k: usize, av: Vec<usize>, xyv: Vec<(i64, i64)>) -> Self {
        let start_time = Instant::now();
        let mut horizontal_points = HashSet::new();
        let mut vertical_points = HashSet::new();
        let edge = 10.pow(9) + 2;
        horizontal_points.insert(-edge);
        horizontal_points.insert(edge);
        vertical_points.insert(-edge);
        vertical_points.insert(edge);
        for (x, y) in &xyv {
            horizontal_points.insert(*y);
            vertical_points.insert(*x);
        }
        let compressed_horizontal =
            compress_zahyo(&horizontal_points.iter().copied().collect::<Vec<_>>());
        let compressed_vertical =
            compress_zahyo(&vertical_points.iter().copied().collect::<Vec<_>>());
        let mut counts = vec![vec![0; vertical_points.len() + 1]; horizontal_points.len() + 1];
        for (x, y) in &xyv {
            let i = *compressed_horizontal.get(y).unwrap();
            let j = *compressed_vertical.get(x).unwrap();
            counts[i][j] += 1;
        }
        let mut cusum = vec![vec![0; vertical_points.len() + 1]; horizontal_points.len() + 1];
        for i in 0..horizontal_points.len() {
            for j in 0..vertical_points.len() {
                cusum[i + 1][j + 1] =
                    cusum[i + 1][j] + cusum[i][j + 1] - cusum[i][j] + counts[i][j];
            }
        }
        Solver {
            n,
            k,
            av,
            strawberry_list: xyv,
            strawberry_cusum: cusum,
            compressed_strawberry_horizontal: compressed_horizontal,
            compressed_strawberry_vertical: compressed_vertical,
            start_time,
            rng: rand::thread_rng(),
        }
    }

    pub fn solve(&mut self) {
        // 初期盤面を登録
        let mut scores = Scores::new(10);
        let mut horizontal_lines = vec![].into_iter().collect::<BTreeSet<_>>();
        let mut vertical_lines = vec![].into_iter().collect::<BTreeSet<_>>();
        let initial_score = ScoreCalculator::calculate(
            &self.av,
            &self.strawberry_cusum,
            &self.compressed_strawberry_horizontal,
            &self.compressed_strawberry_vertical,
            &mut horizontal_lines,
            &mut vertical_lines,
            false,
        );
        scores.update_if_needed(0, &horizontal_lines, &vertical_lines);

        let mut max_score = (
            initial_score,
            (horizontal_lines.clone(), vertical_lines.clone()),
        );

        // スコアの高い盤面に更新する
        let mut count: usize = 0;
        let depth = 3;
        loop {
            // 時間を確認
            let now = Instant::now();
            let limit = self.start_time + Duration::from_millis(2950);
            if now >= limit {
                break;
            }

            let tmp_scores = scores.get_scores();
            scores = Scores::new(10);
            for (s, (mut horizontal, mut vertical)) in tmp_scores {
                if s > max_score.0 {
                    max_score = (s, (horizontal.clone(), vertical.clone()));
                }

                let max_depth = (K - horizontal.len() - vertical.len()).min(depth);
                self.solve_dfs(
                    &mut count,
                    0,
                    &mut scores,
                    &mut horizontal,
                    &mut vertical,
                    max_depth,
                    &limit,
                );
            }
        }

        let (_score, (horizontal, vertical)) = max_score;
        // ScoreCalculator::calculate(
        //     &self.av,
        //     &self.strawberry_cusum,
        //     &self.compressed_strawberry,
        //     &horizontal,
        //     &vertical,
        //     true,
        // );
        println!("{}", horizontal.len() + vertical.len());
        let edge = 10.pow(9);
        for l in horizontal {
            println!("{} {} {} {}", -edge, l, edge, l);
        }
        for l in vertical {
            println!("{} {} {} {}", l, -edge, l, edge);
        }
        eprintln!("{}", _score);

        if Instant::now() - self.start_time >= Duration::from_millis(3000) {
            panic!("overtime");
        }
    }

    pub fn solve_dfs(
        &mut self,
        c: &mut usize,
        depth: usize,
        scores: &mut Scores,
        horizontal_line: &mut BTreeSet<Line>,
        vertical_line: &mut BTreeSet<Line>,
        max_depth: usize,
        limit: &Instant,
    ) -> bool {
        if depth >= max_depth {
            return true;
        }

        // 時間をチェック
        if *c % 200 == 0 {
            let now = Instant::now();
            if &now >= limit {
                return false;
            }
        }

        // 線をランダムに決める
        let mut try_count = 0;
        let mut add_line: Option<(bool, i64)> = None;
        'outer: loop {
            try_count += 1;
            let si = self.rng.gen::<usize>() % self.strawberry_list.len();
            let s = self.strawberry_list[si];
            let dir = self.rng.gen::<usize>() % 2;
            match dir {
                0 => {
                    let new_line = s.1;
                    if !horizontal_line.contains(&new_line) {
                        horizontal_line.insert(new_line);
                        add_line = Some((true, new_line));
                        break 'outer;
                    }
                }
                1 => {
                    let new_line = s.0;
                    if !vertical_line.contains(&new_line) {
                        vertical_line.insert(new_line);
                        add_line = Some((false, new_line));
                        break 'outer;
                    }
                }
                _ => unreachable!(),
            }
            if try_count > 10 {
                return true;
            }
        }

        *c += 1;
        let new_score = ScoreCalculator::calculate(
            &self.av,
            &self.strawberry_cusum,
            &self.compressed_strawberry_horizontal,
            &self.compressed_strawberry_vertical,
            horizontal_line,
            vertical_line,
            false,
        );

        scores.update_if_needed(new_score, &horizontal_line, &vertical_line);
        let can_continue = self.solve_dfs(
            c,
            depth + 1,
            scores,
            horizontal_line,
            vertical_line,
            max_depth,
            limit,
        );

        match add_line {
            Some((true, l)) => {
                horizontal_line.remove(&l);
            }
            Some((false, l)) => {
                vertical_line.remove(&l);
            }
            _ => unreachable!(),
        }

        if !can_continue {
            return false;
        }
        true
    }
}

fn main() {
    input! {
        n: usize, k: usize,
        av: [usize; 10],
        xyv: [(i64, i64); n]
    };
    let mut solver = Solver::new(n, k, av, xyv);
    solver.solve();
}
