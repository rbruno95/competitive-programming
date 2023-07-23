fn main() {
    let numbers = read_vec::<usize>();
    let (n, m, k) = (numbers[0], numbers[1], numbers[2]);
    let mut applicants = read_vec::<i32>();
    let mut apartments = read_vec::<i32>();

    println!("{}", solve(n, m, k, &mut applicants, &mut apartments));
}

fn solve(
    n: usize,
    m: usize,
    k: usize,
    applicants: &mut Vec<i32>,
    apartments: &mut Vec<i32>,
) -> usize {
    applicants.sort_unstable();
    apartments.sort_unstable();

    let mut counter = 0;
    let mut applicant_idx = 0;
    let mut apartment_idx = 0;

    while applicant_idx < n && apartment_idx < m {
        if (applicants[applicant_idx] - apartments[apartment_idx]).abs() as usize <= k {
            counter += 1;
            applicant_idx += 1;
            apartment_idx += 1;
        } else if applicants[applicant_idx] < apartments[apartment_idx] {
            applicant_idx += 1;
        } else {
            apartment_idx += 1;
        }
    }

    counter
}

#[test]
fn test_solve() {
    assert_eq!(
        solve(4, 3, 5, &mut vec![60, 45, 80, 60], &mut vec![30, 60, 75]),
        2
    );
}

// IO
use std::{fmt::Debug, io, str::FromStr};

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
