use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::HashSet};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse);
    let n: usize = tmp.next().unwrap();
    let m: usize = tmp.next().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let a: HashSet<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let b: HashSet<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

    writeln!(stdout, "{}", n + m - 2 * a.intersection(&b).count()).unwrap();
}
