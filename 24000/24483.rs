use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, cmp::Reverse};
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
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut g = vec![vec![]; n + 1];
    for _ in 0..m {
        let a = tmp.next().unwrap();
        let b = tmp.next().unwrap();
        g[a].push(b);
        g[b].push(a);
    }

    for i in 1..=n {
        g[i].sort_unstable_by_key(|&x| Reverse(x));
    }

    let mut v: Vec<i32> = vec![-1; n + 1];
    let mut c: Vec<i32> = vec![0; n + 1];
    let mut cnt = 1;
    let mut stack: Vec<(usize, i32)> = Vec::new();
    stack.push((s, 0));

    while !stack.is_empty() {
        let tmp = stack.pop().unwrap();

        if v[tmp.0] == -1 {
            v[tmp.0] = tmp.1;
            c[tmp.0] = cnt;
            cnt += 1;
            for &i in g[tmp.0].iter() {
                if v[i] == -1 {
                    stack.push((i, tmp.1 + 1));
                }
            }
        }
    }
    writeln!(stdout, "{}", (1..=n).map(|i| v[i] as i64 * c[i] as i64).sum::<i64>()).unwrap();
}