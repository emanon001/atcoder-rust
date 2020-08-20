#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(PartialEq, Eq)]
enum Type {
    Dict,
    Set,
    Integer,
}

fn parse_expr(s: &[char], pos: &mut usize) -> Type {
    match s[*pos] {
        '{' => {
            // dict | set
            if s[*pos + 1] == '}' {
                *pos += 1;
                return Type::Dict;
            }
            *pos += 1;
            parse_expr(s, pos);
            if s[*pos + 1] == ':' {
                // dict
                *pos += 2;
                parse_expr(s, pos);
                while s[*pos + 1] == ',' {
                    *pos += 2;
                    parse_expr(s, pos);
                    *pos += 2;
                    parse_expr(s, pos);
                }
                *pos += 1;
                Type::Dict
            } else {
                // set
                while s[*pos + 1] == ',' {
                    *pos += 2;
                    parse_expr(s, pos);
                }
                *pos += 1;
                Type::Set
            }
        }
        _ => {
            // integer
            while s[*pos + 1].is_ascii_digit() {
                *pos += 1;
            }
            Type::Integer
        }
    }
}

fn solve() {
    input! {
        s: Chars
    };

    let mut pos = 0;
    let res = if parse_expr(&s, &mut pos) == Type::Dict {
        "dict"
    } else {
        "set"
    };
    println!("{}", res);
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
