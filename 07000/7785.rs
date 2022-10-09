use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::HashSet};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf = buf.split_ascii_whitespace();
    let mut set = HashSet::new();
    while let (Some(name), Some(state)) = (buf.next(), buf.next()) {
        match state {
            "enter" => set.insert(name),
            "leave" => set.remove(name),
            _ => true,
        };
    }
    let mut ans: Vec<&str> = set.into_iter().collect();
    ans.sort_unstable();
    ans.iter().rev().for_each(|s| writeln!(stdout, "{}", s).unwrap());
}