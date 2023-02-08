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

    for i in 1..=n {
        g[i].sort_unstable();
    }

    let mut v: Vec<i32> = vec![-1; n + 1];
    let mut c: Vec<i32> = vec![0; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(s);
    let mut cnt = 1;
    v[s] = 0;
    c[s] = cnt;
    cnt += 1;

    while !queue.is_empty() {
        let tmp = queue.pop_front().unwrap();

        for &i in g[tmp].iter() {
            if v[i] == -1 {
                queue.push_back(i);
                v[i] = v[tmp] + 1;
                c[i] = cnt;
                cnt += 1;
            }
        }
    }
    writeln!(stdout, "{}", (1..=n).map(|i| v[i] as i64 * c[i] as i64).sum::<i64>()).unwrap();
}