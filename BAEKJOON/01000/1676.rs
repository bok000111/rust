use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut n: usize = buf.trim_end().parse().unwrap();

    let mut cnt = 0;
    while n > 0 {
        n /= 5;
        cnt += n;
    }
    writeln!(stdout, "{}", cnt).unwrap();
}