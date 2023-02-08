use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let v = era(10000);

    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let tmp: usize = buf.trim_end().parse().unwrap();
        for i in (2..=tmp / 2).rev() {
            if v[i] && v[tmp - i] {
                writeln!(stdout, "{} {}", i, tmp - i).unwrap();
                break;
            }
        }
    }
}

fn era(n: usize) -> Vec<bool> {
    let mut v: Vec<bool> = vec![true;n + 1];
    (2..=n).for_each(|i| if v[i] {(i..=n / i).for_each(|j| v[i * j] =  false)});
    v[0] = false;
    v[1] = false;
    v
}