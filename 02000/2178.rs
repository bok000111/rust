use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::VecDeque;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let n: i32 = tmp.next().unwrap();
    let m: i32 = tmp.next().unwrap();

    let mut v: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        v.push(Vec::new());
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut tmp = buf.trim().bytes();

        for _ in 0..m
        {
            v[i as usize].push(tmp.next().unwrap() as usize - 48);
        }
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0, 0));
    
    while !q.is_empty() {
        let now = q.pop_front().unwrap();

        if 0 <= now.0 as i32 - 1 {
            if v[now.0 - 1][now.1] == 1 {
                v[now.0 - 1][now.1] = v[now.0][now.1] + 1;
                q.push_back((now.0 - 1 , now.1))
            }
        }
        if now.0 as i32 + 1 < n {
            if v[now.0 + 1][now.1] == 1 {
                v[now.0 + 1][now.1] = v[now.0][now.1] + 1;
                q.push_back((now.0 + 1 , now.1))
            }
        }
        if 0 <= now.1 as i32 - 1 {
            if v[now.0][now.1 - 1] == 1 {
                v[now.0][now.1 - 1] = v[now.0][now.1] + 1;
                q.push_back((now.0, now.1 - 1))
            }
        }
        if now.1 as i32 + 1 < m {
            if v[now.0][now.1 + 1] == 1 {
                v[now.0][now.1 + 1] = v[now.0][now.1] + 1;
                q.push_back((now.0, now.1 + 1))
            }
        }
    }
    writeln!(stdout, "{}", v[n as usize - 1][m as usize - 1]).unwrap();
}