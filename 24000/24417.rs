use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    let mut a = 1;
    let mut b = 1;
    for _ in 3..=n {
        let tmp = (a + b) % 1000000007;
        a = b;
        b = tmp;
    }
    writeln!(stdout, "{} {}", b, n - 2).unwrap();
}