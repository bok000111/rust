use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::cmp::Ordering;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let _k = tmp.next().unwrap();
    let n = tmp.next().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let arr: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse::<usize>).collect();

    let mut s: usize = 1;
    let mut e = *arr.iter().max().unwrap();
    while s <= e {
        let m = (s + e) >> 1;

        let tmp: usize = arr.iter().map(|&n| n.saturating_sub(m)).sum();

        match tmp.cmp(&n) {
            Ordering::Less => e = m - 1,
            Ordering::Equal | Ordering::Greater => s = m + 1,
        }
    }
    writeln!(stdout, "{}", e).unwrap();
}