use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::VecDeque;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: i32 = buf.trim_end().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
        let n: i32 = tmp.next().unwrap();
        let m: i32 = tmp.next().unwrap();
        let c: i32 = tmp.next().unwrap();

        let mut v: Vec<Vec<usize>> = vec![vec![0;m as usize];n as usize];
        for _ in 0..c {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
            v[tmp.next().unwrap()][tmp.next().unwrap()] = 1;
        }

        let mut cnt = 0;
        for i in 0..n {
            for j in 0..m {
                cnt += bfs(&mut v, (i as usize, j as usize), n, m);
            }
        }
        writeln!(stdout, "{}", cnt).unwrap();
    }
}

fn bfs(v: &mut Vec<Vec<usize>>, (r, c): (usize, usize), n: i32, m: i32) -> i32 {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    if v[r][c] == 1 {
        q.push_back((r, c));
        
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
        return 1;
    } else {
        return 0;
    }
}