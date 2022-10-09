use std::io::{stdout, stdin, BufWriter, Write, Read};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse);
    while let Some(n) = buf.next() {
        let n = 3_usize.pow(n);

        for i in 0..n {
            write!(stdout, "{}", rec(n, i)).unwrap();
        }
        writeln!(stdout, "").unwrap();
    }
}

fn rec(n: usize, s: usize) -> char {
    if n == 1 {
        match s {
            0 | 2 => return '-',
            1 => return ' ',
            _ => ' ',
        }
    } else {
        if n / 3 <= s && s < n / 3 * 2 {
            return ' ';
        } else {
            return rec(n / 3, s % (n / 3));
        }
    }
}