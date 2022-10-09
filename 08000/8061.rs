use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::VecDeque};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let nm: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

    let mut g: Vec<Vec<i32>> = vec![vec![0; nm[1]]; nm[0]];
    let mut q: VecDeque<_> = VecDeque::new();

    buf.clear();
    for i in 0..nm[0] {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.trim().bytes().map(|x| x as i32 - 48);
        for j in 0..nm[1] {
            let a: i32 = tmp.next().unwrap();
            if a == 1 {
                q.push_back((i as i32, j as i32));
            }
            g[i][j] = a;
        }
    }

    let d0 = [1, -1, 0, 0];
    let d1 = [0, 0, 1, -1];

    while let Some(now) = q.pop_front() {        
        for i in 0..4 {
            let next_i = now.0 + d0[i];
            let next_j = now.1 + d1[i];

            if 0 <= next_i && next_i < nm[0] as i32 && 0 <= next_j && next_j < nm[1] as i32 {
                if g[next_i as usize][next_j as usize] == 0 {
                    g[next_i as usize][next_j as usize] = g[now.0 as usize][now.1 as usize] + 1;
                    q.push_back((next_i, next_j));
                }
            }
        }
    }
    for i in 0..nm[0] {
        for j in 0..nm[1] {
            write!(stdout, "{} ", g[i][j] - 1).unwrap();
        }
        writeln!(stdout, "").unwrap();
    }
}