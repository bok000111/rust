use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    let mut v = vec![0;n + 1];

    for tmp in 2..=n {
        v[tmp] = v[tmp - 1] + 1;
        if tmp % 2 == 0 {
            v[tmp] = std::cmp::min(v[tmp / 2] + 1, v[tmp]);
        }
        if tmp % 3 == 0 {
            v[tmp] = std::cmp::min(v[tmp / 3] + 1, v[tmp]);
        }
    }
    writeln!(stdout, "{}", v[n]).unwrap();
}