use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, cmp::Reverse};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let _: usize = buf.trim_end().parse().unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut arr: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    arr.sort_unstable_by_key(|&x| Reverse(x));
    let mut ans: usize = 0;

    while let Some(tmp) = arr.pop() {
        ans += tmp * (arr.len() + 1);
    }
    writeln!(stdout, "{}", ans).unwrap();
}
