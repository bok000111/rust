use std::io::{stdout, stdin, BufRead, BufWriter, Write};
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
        g[i].sort_unstable();
    }

    let mut v: Vec<i32> = vec![0; n + 1];
    let mut stack: Vec<usize> = Vec::new();
    stack.push(s);
    let mut cnt = 1;

    while !stack.is_empty() {
        let tmp = stack.pop().unwrap();

        if v[tmp] == 0 {
            v[tmp] = cnt;
            cnt += 1;
            for &i in g[tmp].iter() {
                if v[i] == 0 {
                    stack.push(i);
                }
            }
        }
    }
    (1..=n).for_each(|i| writeln!(stdout, "{}", v[i]).unwrap());
}