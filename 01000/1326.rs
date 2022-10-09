use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::VecDeque};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let g: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    let mut v: Vec<i32> = vec![-1; n];

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let se: Vec<usize> = buf.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect();

    let mut q = VecDeque::new();

    q.push_back(se[0]);
    v[se[0]] = 0;

    while let Some(now) = q.pop_front() {
        if now == se[1] {
            break;
        }
        let mut i = now % g[now];
        while i < n {
            if v[i] == -1 {
                v[i] = v[now] + 1;
                q.push_back(i)
            }
            i += g[now];
        }
    }

    writeln!(stdout, "{}", v[se[1]]).unwrap();
}