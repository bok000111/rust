use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();
    let mut v = vec![vec![0_usize;31];31];

    for i in 0..=30 {
        v[0][i] = 1;
        v[i][i] = 1;
    }
    for i in 1..=30 {
        for j in 1..=30 {
            v[i][j] = v[i - 1][j - 1] + v[i][j - 1];
        }
    }
    buf.clear();
    for _ in 0..t {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    for _ in 0..t {
        writeln!(stdout, "{}", v[tmp.next().unwrap()][tmp.next().unwrap()]).unwrap();
    }
}