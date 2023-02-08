use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::cmp::min;
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
    let mut f: Vec<i32> = buf.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    f.push(0);
    f.push(0);
    let mut ans: i32 = 0;
    for i in 0..n
    {
        if f[i + 1] > f[i + 2]
        {
            let mut tmp = min(f[i], f[i + 1] - f[i + 2]);
            ans += tmp * 5;
            f[i] -= tmp;
            f[i + 1] -= tmp;
            tmp = min(f[i], min(f[i + 1], f[i + 2]));
            ans += tmp * 7;
            f[i] -= tmp;
            f[i + 1] -= tmp;
            f[i + 2] -= tmp;
        }
        else
        {
            let mut tmp = min(f[i], min(f[i + 1], f[i + 2]));
            ans += tmp * 7;
            f[i] -= tmp;
            f[i + 1] -= tmp;
            f[i + 2] -= tmp;
            tmp = min(f[i], f[i + 1]);
            ans += tmp * 5;
            f[i] -= tmp;
            f[i + 1] -= tmp;
        }
        ans += f[i] * 3;
    }
    writeln!(stdout, "{}", ans).unwrap();
}