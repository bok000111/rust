use std::io::{stdout, stdin, BufWriter, Write, Read};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut buf: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse::<i32>).skip(1).collect();
    buf.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    buf.iter().for_each(|&x| writeln!(stdout, "{}", x).unwrap());
}