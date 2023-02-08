use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();
    let mut v: Vec<(i32, i32)> = Vec::new();
    buf.clear();
    for _i in 0..t
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf = buf.split_ascii_whitespace();
    for _i in 0..t
    {
        v.push((buf.next().unwrap().parse::<i32>().unwrap(), buf.next().unwrap().parse::<i32>().unwrap()));
    }
    v.sort_unstable_by(|a, b| a.0.cmp(&(b.0)).then(a.1.cmp(&b.1)));
    v.iter().for_each(|x| writeln!(stdout, "{} {}", x.0, x.1).unwrap());
}