use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

    let v = era(n[1]);

    writeln!(stdout, "{}", (n[0]..=n[1]).filter(|&x| v[x] && is_d(x, n[2])).count()).unwrap();
}

fn era(n: usize) -> Vec<bool> {
    let mut v: Vec<bool> = vec![true;n + 1];
    v[1] = false;
    (2..=n).for_each(|i| if v[i] {(i..=n / i).for_each(|j| v[i * j] =  false)});
    v
}

fn is_d(x: usize, n: usize) -> bool {
    x.to_string().matches(&n.to_string()).count() > 0
}