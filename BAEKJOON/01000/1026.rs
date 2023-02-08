use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut a: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    a.sort_unstable();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut b: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    b.sort_unstable();

    writeln!(stdout, "{}", a.iter().zip(b.iter().rev()).map(|(&a, &b)| a * b).sum::<i32>()).unwrap();
}