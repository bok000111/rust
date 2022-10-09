use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let v: Vec<u8> = buf.trim_end().bytes().collect();

    let a = v[0] ^ 'C' as u8;

    v.iter().for_each(|&c| write!(stdout, "{}", (c ^ a) as char).unwrap());
    writeln!(stdout, "").unwrap()
}