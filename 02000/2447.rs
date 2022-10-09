use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim_end().parse().unwrap();

    for i in 0..n {
        for j in 0..n {
            write!(stdout, "{}", rec(n, i, j)).unwrap();
        }
        writeln!(stdout, "").unwrap();
    }

}

fn rec(n: usize, i: usize, j: usize) -> char {
    if n == 3 {
        match (i, j) {
            (1, 1) => return ' ',
            _ => '*',
        }
    } else {
        if n / 3 <= i && i < n / 3 * 2 && n / 3 <= j && j < n / 3 * 2 {
            return ' ';
        } else {
            return rec(n / 3, i % (n / 3), j % (n / 3));
        }
    }
}