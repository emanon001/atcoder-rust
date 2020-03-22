use proconio::input;

fn main() {
  input! {
    s: String
  };

  // regex crate を含めるとサイズオーバーになる
  // A KIH A B A R A
  let mut candidates = Vec::new();
  for a1 in vec![false, true] {
    let mut c = "".to_string();
    if a1 {
      c.push('A');
    }
    c.push_str("KIH");
    for a2 in vec![false, true] {
      let mut c = c.clone();
      if a2 {
        c.push('A')
      }
      c.push_str("B");
      for a3 in vec![false, true] {
        let mut c = c.clone();
        if a3 {
          c.push('A')
        }
        c.push_str("R");
        for a4 in vec![false, true] {
          let mut c = c.clone();
          if a4 {
            c.push('A')
          }
          candidates.push(c);
        }
      }
    }
  }

  if candidates.contains(&s) {
    println!("YES");
  } else {
    println!("NO");
  }
}
