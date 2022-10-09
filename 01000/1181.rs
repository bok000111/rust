use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim_end().parse().unwrap();

    buf.clear();
    for _i in 0..n
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut v: Vec<&str> = buf.split_ascii_whitespace().collect();

    v.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    v.dedup();
    writeln!(stdout, "{}", v.join("\n")).unwrap();
}