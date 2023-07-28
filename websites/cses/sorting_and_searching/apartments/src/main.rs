use std::{io, str};

// TEMPLATE
struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}
impl<R: io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
// TEMPLATE (END)

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, w: &mut W) {
    let numbers = (0..3).map(|_| scan.token::<usize>()).collect::<Vec<_>>();
    let (n, m, k) = (numbers[0], numbers[1], numbers[2]);
    let mut applicants = (0..n).map(|_| scan.token::<i64>()).collect::<Vec<_>>();
    let mut apartments = (0..m).map(|_| scan.token::<i64>()).collect::<Vec<_>>();

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

    writeln!(w, "{counter}").ok();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
