use std::io::{stdout, stdin, BufRead, BufWriter, Write};

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
    for _ in 0..n
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut arr = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut v = Vec::new();
    for _ in 0..n
    {
        let tmp = arr.next().unwrap();
        if tmp == 0
        {
            v.pop().unwrap();
        }
        else {
            v.push(tmp);
        }
    }
        writeln!(stdout, "{}", v.iter().sum::<i32>()).unwrap();
}