use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let _: usize = tmp.next().unwrap();
    let k: usize = tmp.next().unwrap();


    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut buf: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse::<i32>).collect();
    buf.sort_unstable();
    writeln!(stdout, "{}", buf[k - 1]).unwrap();
}