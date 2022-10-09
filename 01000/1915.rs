use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, cmp::min};
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
    let mut v: Vec<Vec<i32>> = vec![];
    let mut max = 0;
    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        v.push(buf.trim_end().bytes().map(|x| x as i32 - 48).collect());
    }

    for i in 1..n {
        for j in 1..m {
            if v[i][j] != 0 {
                v[i][j] = min(v[i - 1][j - 1], min(v[i][j - 1], v[i - 1][j])) + 1;
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            if v[i][j] > max {
                max = v[i][j];
            }
        }
    }
    writeln!(stdout, "{}", max * max).unwrap();
}
