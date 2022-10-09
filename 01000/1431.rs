use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim_end().parse().unwrap();

    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf: Vec<_> = buf.split_ascii_whitespace().collect();
    buf.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then(a.chars().filter_map(|x| x.to_digit(10)).sum::<u32>().cmp(&b.chars().filter_map(|x| x.to_digit(10)).sum::<u32>())).then(a.cmp(&b)));
    for str in buf {
        writeln!(stdout, "{}", str).unwrap();
    }
}