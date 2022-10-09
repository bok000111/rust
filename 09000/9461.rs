use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut v: Vec<usize> = vec![0; 101];
    v[1] = 1;
    v[2] = 1;
    v[3] = 1;
    v[4] = 2;
    v[5] = 2;
    for i in 6..=100 {
        v[i] = v[i - 1] + v[i - 5];
    }
    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        writeln!(stdout, "{}", v[buf.trim_end().parse::<usize>().unwrap()]).unwrap();
    }
}