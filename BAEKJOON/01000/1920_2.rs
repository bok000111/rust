use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let _n: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut num: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    num.sort_unstable();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let m: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut num2 = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _ in 0..m
    {
        if num.binary_search(&num2.next().unwrap()).is_ok()
        {
            writeln!(stdout, "1").unwrap();
        }
        else
        {
            writeln!(stdout, "0").unwrap();
        }
    }
}