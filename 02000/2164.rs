use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::VecDeque;

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = tmp.next().unwrap();

    let mut v:VecDeque<usize> = (1..=n).collect();

    for _ in 1..n
    {
        v.pop_front().unwrap();
        let tmp = v.pop_front().unwrap();
        v.push_back(tmp);
    }
    
    writeln!(stdout, "{}", v[0]).unwrap();
}