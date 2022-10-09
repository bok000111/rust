use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::VecDeque};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let nm: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    let mut g: Vec<_> =  vec![];
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut tomato = 0;
    let mut cnt = 0;
    let mut day = 0;

    buf.clear();
    for _ in 0..nm[2] {
        for _ in 0..nm[1] {
            stdin.read_line(&mut buf).unwrap();
        }
    }
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    for i in 0..nm[2] {
        g.push(vec![]);
        for j in 0..nm[1] {
            g[i].push(vec![]);
            for k in 0..nm[0] {
                let tmp = buf.next().unwrap();
                g[i][j].push(tmp);
                if tmp == 1 {
                    tomato += 1;
                    cnt += 1;
                    q.push_back((i, j, k));
                } else if tmp == 0 {
                    tomato += 1;
                }
            }
        }
    }

    let da = [1, -1, 0, 0, 0, 0];
    let db = [0, 0, 1, -1, 0, 0];
    let dc = [0, 0, 0, 0, 1, -1];

    while let Some((a, b, c)) = q.pop_front() {
        for i in 0..6 {
            let next_a = a as i32 + da[i];
            let next_b = b as i32 + db[i];
            let next_c = c as i32 + dc[i];

            if 0 <= next_a && next_a < nm[2] as i32 && 0 <= next_b && next_b < nm[1] as i32 && 0 <= next_c && next_c < nm[0] as i32 {
                if g[next_a as usize][next_b as usize][next_c as usize] == 0 {
                    q.push_back((next_a as usize, next_b as usize, next_c as usize));
                    cnt += 1;
                    g[next_a as usize][next_b as usize][next_c as usize] = g[a][b][c] + 1;
                    if day < g[a][b][c] {
                        day = g[a][b][c];
                    }
                }
            }
        }
    }

    if cnt != tomato {
        writeln!(stdout, "-1").unwrap();
    } else {
        writeln!(stdout, "{}", day).unwrap();
    }
}