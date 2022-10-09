use std::{io::{stdout, stdin, BufRead, BufWriter, Write}, iter::successors};
fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    writeln!(stdout, "{}", (666..).filter(|n| is_666(*n)).nth(n - 1).unwrap()).unwrap();
}

fn is_666(num: i32) -> bool
{
    successors(Some(num), |n| Some(*n / 10))
        .take_while(|n| *n >= 666)
        .any(|n| n % 1000 == 666)
}