use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::VecDeque;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let tmp: Vec<char> = buf.trim_end().chars().collect();
        if tmp.len() == 1 {
            break;
        }
        writeln!(stdout, "{}", is_balance(&tmp)).unwrap();
    }
}

fn is_balance(v: &Vec<char>) -> &str {
    let mut s: VecDeque<char> = VecDeque::new();

    for &i in v {
        match i {
            '(' => s.push_back('('),
            '[' => s.push_back('['),
            ')' => {
                match s.pop_back().unwrap_or('a') {
                    '(' => (),
                    _ => return "no",
                }
            },
            ']' => {
                match s.pop_back().unwrap_or('a') {
                    '[' => (),
                    _ => return "no",
                }
            },
            '.' => break,
            _ => (),
        };
    }
    match s.len() {
        0 => "yes",
        _ => "no",
    }
}