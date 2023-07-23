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
use std::{fmt::Debug, io, str::FromStr};

fn read<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input.trim().parse().expect("Error parsing the line")
}

fn read_vec<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Error parsing the line"))
        .collect()
}
