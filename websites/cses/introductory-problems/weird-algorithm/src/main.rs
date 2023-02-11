fn main() {
    let n = read_i64();

    println!("{}", solve(n));
}

fn solve(mut n: i64) -> String {
    let mut values = Vec::new();

    loop {
        values.push(n);

        if n == 1 {
            break;
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn test_solve() {
    assert_eq!(solve(3).as_str(), "3 10 5 16 8 4 2 1")
}

// IO
use std::io;

fn read_i64() -> i64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input.trim().parse().unwrap()
}
