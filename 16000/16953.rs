use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::VecDeque};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut t = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let a = t.next().unwrap();
    let b = t.next().unwrap();

    let mut q: VecDeque<(usize, i32)> = VecDeque::new();

    q.push_back((a, 1));
    let mut ans = -1;
    while let Some(now) = q.pop_front() {
        match now.0.cmp(&b) {
            std::cmp::Ordering::Less => {
                if now.0 * 2 <= b {q.push_back((now.0 * 2, now.1 + 1));};
                if now.0 * 10 + 1 <= b {q.push_back((now.0 * 10 + 1, now.1 + 1));};
            }
            std::cmp::Ordering::Equal => {
                ans = now.1;
                break;
            }
            std::cmp::Ordering::Greater => (),
        }
    }
    writeln!(stdout, "{}", ans).unwrap();
}