fn main() {
    let n = read::<i64>();
    let numbers = read_vec::<i64>();

    println!("{}", solve(n, &numbers));
}

fn solve(n: i64, numbers: &[i64]) -> i64 {
    n * (n + 1) / 2 - numbers.iter().sum::<i64>()
}

#[test]
fn test_solve() {
    assert_eq!(solve(5, &[2, 3, 1, 5]), 4);
}

// IO
use std::{io, str::FromStr};

fn read<T: FromStr>() -> T {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("Error parsing the line"))
}

fn read_vec<T: FromStr>() -> Vec<T> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|_| panic!("Error parsing the line"))
        })
        .collect()
}
