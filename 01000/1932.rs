use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    let mut v: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        v.push(buf.split_ascii_whitespace().flat_map(str::parse).collect());
    }

    for i in (0..n - 1).rev() {
        for j in 0..=i {
            v[i][j] = std::cmp::max(v[i + 1][j] + v[i][j], v[i + 1][j + 1] + v[i][j])
        }
    }
    writeln!(stdout, "{}", v[0][0]).unwrap();
}