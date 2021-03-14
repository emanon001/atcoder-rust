#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Instant, Duration};
use rand::prelude::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

#[derive(Copy, Clone)]
struct Ad {
    id: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    r: i64,
    xy: (usize, usize),
}

impl Ad {
    fn point(&self) -> f64 {
        let Ad { id: _, a, b, c, d, r, xy } = *self;
        let x = xy.0;
        let y = xy.1;
        if a <= x && x <= c && b <= y && y <= d {
            let s = (c - a) * (d - b);
            let m = 1.0 - i64::min(r, s as i64) as f64 / i64::max(r, s as i64) as f64;
            1.0 - m * m
        } else {
            0.0
        }
    }

    fn is_overlapping(&self, ad: &Self) -> bool {
        let ok = self.a >= ad.c || self.b >= ad.d || self.c <= ad.a || self.d <= ad.b;
        return !ok;
    }
}

fn score(ads: &[Ad]) -> i64 {
    let mut sum = 0_f64;
    for ad in ads {
        sum += ad.point() / ads.len() as f64;
    }
    (10.pow(9) as f64 * sum).round() as i64
}

fn update(ads: &mut [Ad], new_ad: Ad, bk_ad: Ad, max_score: i64) -> i64 {
    let is_ok = ads.iter().all(|oad| {
        if oad.id == new_ad.id {
            return true;
        }
        return !new_ad.is_overlapping(oad);
    });
    if !is_ok {
        return max_score;
    }
    ads[bk_ad.id] = new_ad;
    let new_score = score(&ads);
    if new_score > max_score {
        return new_score;
    } else {
        ads[bk_ad.id] = bk_ad;
        return max_score;
    }
}

fn solve() {
    input! {
        n: usize,
        xyrv: [(usize, usize, i64); n]
    };

    let now = Instant::now();
    let mut ads = Vec::new();
    let inputs = xyrv.into_iter().enumerate().collect::<Vec<_>>();
    for &(i, (x, y, r)) in &inputs {
        ads.push(Ad {
            id: i,
            a: x,
            b: y,
            c: x + 1,
            d: y + 1,
            r,
            xy: (x, y)
        });
    }
    let mut max_score = score(&ads);
    let h = 10000_usize;
    let w = 10000_usize;

    let mut rng = rand::thread_rng();
    loop {
        let duration = Instant::now() - now;
        if duration >= Duration::from_millis(4800) {
            break;
        }

        let i = rng.gen::<usize>() % n;
        let dir = rng.gen::<usize>() % 4;
        let bk_ad = ads[i];
        let mut new_ad = ads[i];
        // 左
        if new_ad.a > 0 && dir == 0 {
            new_ad.a -= 1;
            max_score = update(&mut ads, new_ad, bk_ad, max_score);
        }
        // 右
        if new_ad.c < w - 1 && dir == 1 {
            new_ad.c += 1;
            max_score = update(&mut ads, new_ad, bk_ad, max_score);
        }
        // 上
        if new_ad.b > 0 && dir == 2 {
            new_ad.b -= 1;
            max_score = update(&mut ads, new_ad, bk_ad, max_score);
        }
        // 下
        if new_ad.d < h - 1 && dir == 3 {
            new_ad.d += 1;
            max_score = update(&mut ads, new_ad, bk_ad, max_score);
        }
    }

    for ad in ads {
        println!("{} {} {} {}", ad.a, ad.b, ad.c, ad.d);
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
