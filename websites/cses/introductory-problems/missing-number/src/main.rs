fn main() {
    let n = read_i64();
    let numbers = read_vec_i64();

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
use std::io;

fn read_i64() -> i64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input
        .trim()
        .parse()
        .expect("Error parsing the string to i64")
}

fn read_vec_i64() -> Vec<i64> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Error parsing the string to i64"))
        .collect()
}
