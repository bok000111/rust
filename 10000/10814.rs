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
    let mut v: Vec<Vec<&str>> = Vec::new();
    buf.clear();
    for _i in 0..t
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf = buf.split_ascii_whitespace();
    for _i in 0..t
    {
        v.push(vec![buf.next().unwrap(), buf.next().unwrap()]);
    }
    v.sort_by(|a, b|a[0].parse::<i32>().unwrap().cmp(&(b[0].parse::<i32>().unwrap())));
    writeln!(stdout, "{}", v.into_iter().map(|x| x.join(" ")).collect::<Vec<String>>().join("\n")).unwrap();
}