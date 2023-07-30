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
        let (n, k) = scan_tuple!(scan, usize, usize);
        let c = scan.vector::<usize>(n);

        let sol = if c[0] == c[n - 1] {
            if c.iter().filter(|&&color| color == c[0]).count() >= k {
                "YES"
            } else {
                "NO"
            }
        } else {
            match (
                (0..n).filter(|&i| c[i] == c[0]).nth(k - 1),
                (0..n).rev().filter(|&i| c[i] == c[n - 1]).nth(k - 1),
            ) {
                (Some(i), Some(j)) if i < j => "YES",
                _ => "NO",
            }
        };

        writeln!(w, "{sol}").ok();
    })
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
