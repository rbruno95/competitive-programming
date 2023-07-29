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
    fn vector<T: str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.token::<T>()).collect::<Vec<_>>()
    }
}

macro_rules! scan_tuple {
    ($x:expr, $($t:ty),*) => {
        ($($x.token::<$t>(),)*)
    };
}
// TEMPLATE (END)

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, w: &mut W) {
    let testcases = scan.token::<usize>();

    (0..testcases).for_each(|_| {
        let (n, m, k, vlad_h) = scan_tuple!(scan, usize, i32, i32, i32);
        let people_h = scan.vector::<i32>(n);

        let sol = (0..n)
            .filter(|&i| {
                people_h[i] != vlad_h
                    && ((people_h[i] - vlad_h) % k == 0)
                    && (people_h[i] - vlad_h).abs() / k <= (m - 1)
            })
            .count();

        writeln!(w, "{sol}").ok();
    })
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
