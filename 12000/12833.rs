use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let v: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

    if v[2] & 1 == 0 {
        writeln!(stdout, "{}", v[0]).unwrap();
    } else {
        writeln!(stdout, "{}", v[0] ^ v[1]).unwrap();
    }
}