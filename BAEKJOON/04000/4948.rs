use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();

    let mut v: Vec<bool> = vec![true;246913];
    (2..=246912).for_each(|i| if v[i] {(i..=246912 / i).for_each(|j| v[i * j] =  false)});

    loop {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let n: usize = buf.trim_end().parse().unwrap();
        if n == 0 {
            break;
        }
        writeln!(stdout, "{}", (n + 1..=2 * n).filter(|&i| v[i]).count()).unwrap();
    }
}