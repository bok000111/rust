use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
    }
    let arr: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse::<usize>).collect();

    let mut v0: Vec<usize> = vec![0;41];
    let mut v1: Vec<usize> = vec![0;41];
    v0[0] = 1;
    v1[1] = 1;

    for i in 2..=40 {
        v0[i] = v0[i - 1] + v0[i - 2];
        v1[i] = v1[i - 1] + v1[i - 2];
    }
    for i in arr {
        writeln!(stdout, "{} {}", v0[i], v1[i]).unwrap();
    }
}