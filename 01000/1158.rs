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
    let k = tmp.next().unwrap();

    let mut v:VecDeque<usize> = (1..=n).collect();
    let mut ans:Vec<usize> = Vec::new();

    while !v.is_empty()
    {
        for _ in 1..k
        {
            let tmp = v.pop_front().unwrap();
            v.push_back(tmp);
        }
        ans.push(v.pop_front().unwrap());
    }
    write!(stdout, "<{}", ans[0]).unwrap();
    for i in 1..n
    {
        write!(stdout, ", {}", ans[i]).unwrap();
    }
    writeln!(stdout, ">").unwrap();
}