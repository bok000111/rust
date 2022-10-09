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

    let mut v:Vec<(i32, i32)> = Vec::new();

    for _ in 0..n
    {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse);
        v.push((tmp.next().unwrap(), tmp.next().unwrap()));
    }
    let mut ans = Vec::new();

    for i in 0..n
    {
        let mut cnt = 1;
        for j in 0..n
        {
            if v[i].0 < v[j].0 && v[i].1 < v[j].1
            {
                cnt += 1;
            }
        }
        ans.push(cnt);
    }
    for i in ans
    {
        writeln!(stdout, "{}", i).unwrap();
    }
}