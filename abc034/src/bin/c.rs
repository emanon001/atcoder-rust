use proconio::input;

pub struct ModComb {
  max: usize,
  modulo: usize,
  fac: Vec<usize>,
  finv: Vec<usize>,
}

impl ModComb {
  pub fn new(max: usize, modulo: usize) -> Self {
    let mut fac = vec![0_usize; max + 1];
    let mut finv = vec![0_usize; max + 1];
    let mut inv = vec![0_usize; max + 1];
    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for i in 2..=max {
      fac[i] = (fac[i - 1] * i) % modulo;
      inv[i] = modulo - ((inv[modulo % i] * (modulo / i)) % modulo);
      finv[i] = (finv[i - 1] * inv[i]) % modulo;
    }
    Self {
      max,
      modulo,
      fac,
      finv,
    }
  }

  pub fn comb(&self, n: usize, k: usize) -> usize {
    if n > self.max {
      panic!();
    }
    if n < k {
      return 0;
    }
    (self.fac[n] * ((self.finv[k] * self.finv[n - k]) % self.modulo)) % self.modulo
  }
}

fn main() {
  input! {
    w: usize, h: usize
  };

  let comb = ModComb::new(h + w, 1_000_000_007);
  let res = comb.comb(w + h - 2, w - 1);
  println!("{}", res);
}
