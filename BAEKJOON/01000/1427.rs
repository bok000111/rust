use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf: Vec<char> = buf.trim_end().chars().collect();
    buf.sort_unstable();
    writeln!(stdout, "{}", buf.iter().rev().collect::<String>()).unwrap();
}
