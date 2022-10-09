use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::HashSet};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    stdin.read_line(&mut buf).unwrap();
    let mut n = buf.split_ascii_whitespace().flat_map(str::parse::<i32>).skip(1);

    let mut set: HashSet<i32> = HashSet::new();

    while let Some(tmp) = n.next() {
        set.insert(tmp);
    }

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    stdin.read_line(&mut buf).unwrap();
    let mut m = buf.split_ascii_whitespace().flat_map(str::parse::<i32>).skip(1);

    while let Some(tmp) = m.next() {
        match set.contains(&tmp) {
            true => write!(stdout, "{} ", 1).unwrap(),
            false => write!(stdout, "{} ", 0).unwrap(),
        }
    }
    writeln!(stdout, "").unwrap()
}
