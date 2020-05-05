use proconio::input;

fn main() {
  input! {
    n: usize,
    av: [usize; n],
    bv: [usize; n]
  };

  let asum = av.iter().sum::<usize>();
  let bsum = bv.iter().sum::<usize>();
  if asum > bsum {
    println!("No");
    std::process::exit(0);
  }
  let op_c = bsum - asum;
  let mut c1 = 0_usize;
  let mut c2 = 0_usize;
  for i in 0..n {
    let a = av[i];
    let b = bv[i];
    if a > b {
      c1 += a - b;
    } else {
      c2 += (b - a + 1) / 2;
    }
  }
  let res = if c1 <= op_c && c2 <= op_c {
    "Yes"
  } else {
    "No"
  };
  println!("{}", res);
}
