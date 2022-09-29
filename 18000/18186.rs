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
    let p: Vec<usize> = buf.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    

    let p2 = p[1] as i64 + p[2] as i64;
    let p3 = p[1] as i64 + 2 * p[2] as i64;
    let p1 = p[1] as i64;

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut f: Vec<i64> = buf.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    f.push(0);
    f.push(0);
    let mut ans: i64 = 0;

    if p1 <= p2 - p1
    {
        ans = f.iter().sum();
        ans *= p1;
    }
    else
    {
        for i in 0..p[0]
        {
            if f[i + 1] > f[i + 2]
            {
                let mut tmp = min(f[i], f[i + 1] - f[i + 2]);
                ans += tmp * p2;
                f[i] -= tmp;
                f[i + 1] -= tmp;
                tmp = min(f[i], min(f[i + 1], f[i + 2]));
                ans += tmp * p3;
                f[i] -= tmp;
                f[i + 1] -= tmp;
                f[i + 2] -= tmp;
            }
            else
            {
                let mut tmp = min(f[i], min(f[i + 1], f[i + 2]));
                ans += tmp * p3;
                f[i] -= tmp;
                f[i + 1] -= tmp;
                f[i + 2] -= tmp;
                tmp = min(f[i], f[i + 1]);
                ans += tmp * p2;
                f[i] -= tmp;
                f[i + 1] -= tmp;
            }
            ans += f[i] * p1;
        }
    }
    writeln!(stdout, "{}", ans).unwrap();
}