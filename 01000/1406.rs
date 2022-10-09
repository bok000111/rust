use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut lq: Vec<char> = buf.trim_end().chars().collect();
    let mut rq: Vec<char> = Vec::new();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    for _ in 0..n {
        {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.split_ascii_whitespace().flat_map(str::chars);
        match tmp.next().unwrap() {
            'L' => {
                if let Some(tmp) = lq.pop() {
                    rq.push(tmp);
                }
            },
            'D' => {
                if let Some(tmp) = rq.pop() {
                    lq.push(tmp);
                }
            },
            'B' => if let Some(_) = lq.pop() {},
            'P' => lq.push(tmp.next().unwrap()),
            _ => (),
        }
    }
    lq.iter().for_each(|&x| write!(stdout, "{}", x).unwrap());
    rq.iter().rev().for_each(|&x| write!(stdout, "{}", x).unwrap());
    writeln!(stdout, "").unwrap();
}