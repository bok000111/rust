use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let tmp: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse::<i32>).collect();

    let mut num: HashMap<i32, i32> = HashMap::new();
    for i in 0..n
    {
        match num.get(&tmp[i])
        {
            Some(j) => num.insert(tmp[i], j + 1),
            None => num.insert(tmp[i], 1),
        };
    }

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let m: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _i in 0..m
    {
        match num.get(&tmp.next().unwrap())
        {
            Some(i) => write!(stdout, "{} ", i).unwrap(),
            None => write!(stdout, "0 ").unwrap(),
        };
    }
}