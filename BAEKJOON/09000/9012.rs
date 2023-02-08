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

    for _ in 0..n
    {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let str = buf.trim().chars();
        let mut cnt = 0;
        for i in str
        {
            if cnt < 0
            {
                break;
            }
            match i
            {
                '(' => cnt += 1,
                _ => cnt -= 1
            }
        }
        match cnt
        {
            0 => writeln!(stdout, "YES").unwrap(),
            _ => writeln!(stdout, "NO").unwrap()
        }
    }
}