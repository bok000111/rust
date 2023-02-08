use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let _t: usize = buf.trim_end().parse().unwrap();
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let sum:i32 =  buf.split_ascii_whitespace().flat_map(str::parse::<i128>).map(is_prime).sum::<i32>();
    writeln!(stdout, "{}", sum).unwrap();
}

fn is_prime(n: i128) -> i32
{
    if n <= 2
    {
        if n == 2
        {
            return 1;
        }
        return 0;
    }
    for i in 2..=(n as f64).sqrt() as i128
    {
        if n % i == 0
        {
            return 0;
        }
    }
    1
}