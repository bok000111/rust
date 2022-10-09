use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let a = buf.next().unwrap();
    let b = buf.next().unwrap();
    let v = buf.next().unwrap();

    writeln!(stdout, "{}", (v + a - 2 * b - 1) / (a - b)).unwrap();
}