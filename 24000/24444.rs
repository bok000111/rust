use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::VecDeque;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n: usize = tmp.next().unwrap();
    let m: usize = tmp.next().unwrap();
    let s: usize = tmp.next().unwrap();

    buf.clear();
    for _ in 0..m {
        stdin.read_line(&mut buf).unwrap();
    }

    let mut g = vec![vec![]; n + 1];

    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    for _ in 0..m {
        let a = tmp.next().unwrap();
        let b = tmp.next().unwrap();
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..=n {
        g[i].sort_unstable();
    }

    let mut v = vec![0; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut cnt = 1;
    queue.push_back(s);
    v[s] = cnt;
    cnt += 1;

    while !queue.is_empty() {
        let tmp = queue.pop_front().unwrap();

        for &i in g[tmp].iter() {
            if v[i] == 0 {
                queue.push_back(i);
                v[i] = cnt;
                cnt += 1;
            }
        }
    }
    for i in 1..=n {
        writeln!(stdout, "{}", v[i]).unwrap();
    }
}