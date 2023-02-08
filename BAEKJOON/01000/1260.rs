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
        g[i].dedup();
    }

    let mut v = vec![false; n + 1];
    let mut stack: Vec<usize> = Vec::new();
    stack.push(s);

    while !stack.is_empty() {
        let tmp = stack.pop().unwrap();
        if !v[tmp] {
            write!(stdout, "{} ", tmp).unwrap();
            v[tmp] = true;

            for &i in g[tmp].iter().rev() {
                if !v[i] {
                    stack.push(i);
                }
            }
        }
    }
    writeln!(stdout, "").unwrap();

    v = vec![false; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(s);

    while !queue.is_empty() {
        let tmp = queue.pop_front().unwrap();
        if !v[tmp] {
            write!(stdout, "{} ", tmp).unwrap();
            v[tmp] = true;

            for &i in g[tmp].iter() {
                if !v[i] {
                    queue.push_back(i);
                }
            }
        }
    }
    writeln!(stdout, "").unwrap();
}