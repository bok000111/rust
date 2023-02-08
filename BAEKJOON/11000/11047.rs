use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n: usize = tmp.next().unwrap();
    let mut m: usize = tmp.next().unwrap();

    let mut v: Vec<usize> = Vec::new();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        v.push(buf.trim().parse().unwrap());
    }

    let mut cnt = 0;

    for &i in v.iter().rev() {
        cnt += m / i;
        m %= i;
        if m == 0 {
            break;
        }
    }
    writeln!(stdout, "{}", cnt).unwrap();
}