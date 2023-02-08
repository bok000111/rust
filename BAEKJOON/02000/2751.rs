use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t:i32 = buf.trim_end().parse().unwrap();
    buf.clear();
    for _ in 0..t
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut n: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    n.sort_unstable();
    for i in n
    {
        writeln!(stdout, "{}", i).unwrap();
    }
}