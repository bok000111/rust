use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::VecDeque;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim_end().parse().unwrap();

    let mut s: VecDeque<i32> = VecDeque::new();
    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut tmp = buf.split_ascii_whitespace();
        match tmp.next().unwrap() {
            "push_front" => s.push_front(tmp.next().unwrap().parse().unwrap()),
            "push_back" => s.push_back(tmp.next().unwrap().parse().unwrap()),
            "pop_front" => writeln!(stdout, "{}", s.pop_front().unwrap_or(-1)).unwrap(),
            "pop_back" => writeln!(stdout, "{}", s.pop_back().unwrap_or(-1)).unwrap(),
            "size" => writeln!(stdout, "{}", s.len()).unwrap(),
            "empty" => writeln!(stdout, "{}", s.is_empty()).unwrap(),
            "front" => writeln!(stdout, "{}", s.front().unwrap_or(-1)).unwrap(),
            "back" => writeln!(stdout, "{}", s.back().unwrap_or(-1)).unwrap(),
            _ => (),
        }
    }
}