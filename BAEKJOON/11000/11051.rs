use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut v = vec![vec![0_i32;1001];1001];

    for i in 0..=1000 {
        v[i][0] = 1;
        v[i][i] = 1;
    }
    for i in 1..=1000 {
        for j in 1..=1000 {
            v[i][j] = (v[i - 1][j] + v[i - 1][j - 1]) % 10007;
        }
    }
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    writeln!(stdout, "{}", v[tmp.next().unwrap()][tmp.next().unwrap()]).unwrap();
}