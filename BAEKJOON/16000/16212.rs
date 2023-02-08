use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    stdin.read_line(&mut buf).unwrap();
    let mut buf: Vec<i32> = buf.split_ascii_whitespace().skip(1).flat_map(str::parse::<i32>).collect();

    buf.sort_unstable();
    buf.iter().for_each(|&x| write!(stdout, "{} ", x).unwrap());
}