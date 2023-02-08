use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    buf.sort_unstable_by_key(|&x| std::cmp::Reverse(x));

    let mut max = 0;
    for (i, &val) in buf.iter().enumerate() {
        max = std::cmp::max(max, val * (i + 1));
    }
    writeln!(stdout, "{}", max).unwrap();
}