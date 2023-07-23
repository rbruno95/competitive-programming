use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let n = read::<i32>();
    let numbers = read_vec::<i32>();

    println!("{}", solve(n, &numbers));
}

fn solve(_n: i32, numbers: &Vec<i32>) -> usize {
    HashSet::<&i32>::from_iter(numbers).len()
}

#[test]
fn test_solve() {
    assert_eq!(solve(5, &vec![2, 3, 2, 2, 3]), 2);
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
