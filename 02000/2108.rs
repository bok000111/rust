use std::io::{stdout, stdin, BufRead, BufWriter, Write};

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
    for _ in 0..n
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut num: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    num.sort_unstable();

    writeln!(stdout, "{}", (num.iter().sum::<i32>() as f32 / n as f32).round() as i32).unwrap();
    writeln!(stdout, "{}", num[n / 2]).unwrap();

    let mut maxcnt = 0;
    let mut cnt = vec![0;(num[n - 1] - num[0] + 1) as usize];
    for i in 0..n
    {
        cnt[(num[i] - num[0]) as usize] += 1;
        if cnt[(num[i] - num[0]) as usize] > maxcnt
        {
            maxcnt = cnt[(num[i] - num[0]) as usize];
        }
    }
    let mut res = num[0] - 1;
    for (i, &val) in cnt.iter().enumerate()
    {
        if val == maxcnt
        {
            if res == num[0] - 1
            {
                res = i as i32 + num[0];
            }
            else
            {
                res = i as i32 + num[0];
                break;
            }
        }
    }
    writeln!(stdout, "{}", res).unwrap();

    writeln!(stdout, "{}", num[n - 1] - num[0]).unwrap();
}