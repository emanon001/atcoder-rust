use proconio::input;

fn main() {
  input! {
    n: usize,
    d_l: [usize; n],
    m: usize,
    t_l: [usize; m],
  };

  let mut d_table = std::collections::HashMap::new();
  for d in d_l {
    let x = d_table.entry(d).or_insert(0);
    *x += 1;
  }
  for t in t_l {
    if let Some(x) = d_table.get_mut(&t) {
      if *x == 0 {
        println!("NO");
        std::process::exit(0);
      } else {
        *x -= 1;
      }
    } else {
      println!("NO");
      std::process::exit(0);
    }
  }
  println!("YES");
}
