use std::io;

fn main() {
    let mut n = read_i64();

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

    println!(
        "{}",
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn read_i64() -> i64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the line");

    input.trim().parse().unwrap()
}
