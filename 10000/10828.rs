use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim_end().parse().unwrap();

    let mut s: Vec<i32> = Vec::new();
    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut tmp = buf.split_ascii_whitespace();
        match tmp.next().unwrap() {
            "push" => s.push(tmp.next().unwrap().parse().unwrap()),
            "pop" => {
                match s.len() {
                    0 => writeln!(stdout, "-1").unwrap(),
                    _ => writeln!(stdout, "{}", s.pop().unwrap()).unwrap(),
                }
            },
            "size" => writeln!(stdout, "{}", s.len()).unwrap(),
            "empty" => {
                match s.len() {
                    0 => writeln!(stdout, "1").unwrap(),
                    _ => writeln!(stdout, "0").unwrap(),
                }
            }
            "top" => {
                match s.len() {
                    0 => writeln!(stdout, "-1").unwrap(),
                    i => writeln!(stdout, "{}", s[i - 1]).unwrap(),
                }
            }
            _ => (),
        }
    }
}