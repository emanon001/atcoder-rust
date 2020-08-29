#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct FastPrimeFactor {
    n: usize,
    min_primes: Vec<usize>,
}

impl FastPrimeFactor {
    pub fn new(n: usize) -> Self {
        let mut min_primes = vec![0; n + 1];
        for i in 2..=n {
            if min_primes[i] == 0 {
                let mut j = i;
                while j <= n {
                    min_primes[j] = i;
                    j += i;
                }
            }
        }
        Self { n, min_primes }
    }
    pub fn prime_factor(&self, n: usize) -> HashMap<usize, usize> {
        if n < 2 {
            return HashMap::new();
        }
        if n > self.n {
            panic!("n > self.n");
        }
        let mut res = HashMap::new();
        let mut x = n;
        while x > 1 {
            let y = self.min_primes[x];
            *res.entry(y).or_insert(0) += 1;
            x = x / y;
        }
        res
    }
}

fn solve() {
    input! {
        n: usize,
        av: [usize; n]
    };

    let mut set1 = HashSet::new();
    let pf = FastPrimeFactor::new(10.pow(6));
    let mut is_pairwise = true;
    let mut is_setwise = false;
    let mut b = 0;
    for a in av {
        b = b.gcd(&a);
        if b == 1 {
            is_setwise = true;
        }
        for (k, _) in pf.prime_factor(a as usize) {
            if set1.contains(&k) {
                is_pairwise = false;
            }
            set1.insert(k);
        }
    }
    if is_pairwise {
        println!("pairwise coprime");
    } else if is_setwise {
        println!("setwise coprime");
    } else {
        println!("not coprime");
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
