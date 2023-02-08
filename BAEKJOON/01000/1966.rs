use std::{io::{stdout, stdin, BufRead, BufWriter, Write}, collections::VecDeque};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    for _ in 0..t {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.split_ascii_whitespace();
        let n: usize = tmp.next().unwrap().parse().unwrap();
        let m: usize = tmp.next().unwrap().parse().unwrap();
        let mut q = tmp.flat_map(str::parse).zip(0..n).collect::<VecDeque<(usize, usize)>>();

        let mut cnt = 1;
        while let Some(now) = q.get(0) {
            if now.0 < q.iter().map(|&x| x.0).max().unwrap() {
                let tmp = q.pop_front().unwrap();
                q.push_back(tmp);
                continue;
            }
            if now.1 == m {
                break;
            } else {
                q.pop_front().unwrap();
                cnt += 1;
            }
        }
        writeln!(stdout, "{}", cnt).unwrap();
    }
}