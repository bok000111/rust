use std::{io::{stdout, stdin, BufRead, BufWriter, Write}, collections::HashSet};

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut num = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut set = HashSet::new();
    for _ in 0..n
    {
        set.insert(num.next().unwrap());
    }

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let m: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut num2 = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _ in 0..m
    {
        let tmp = num2.next().unwrap();
        if set.contains(&tmp)
        {
            writeln!(stdout, "1").unwrap();
        }
        else
        {
            writeln!(stdout, "0").unwrap();
        }
    }
}