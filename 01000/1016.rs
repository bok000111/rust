use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse);
    let n: i64 = buf.next().unwrap();
    let m: i64 = buf.next().unwrap();
    let mut v: Vec<bool> = vec![true;(m - n + 2) as usize];
    let r = m as f64;
    let r = r.sqrt() as i64;

    for i in 2..=r {
        let tmp = i * i;
        for j in (n + tmp - 1) / tmp..=m / tmp {
            v[(j * tmp - n) as usize] = false;
        }
    }

    writeln!(stdout, "{}", (n..=m).filter(|&x| v[(x - n) as usize]).count()).unwrap();
}