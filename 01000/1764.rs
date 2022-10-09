use std::io::{stdout, stdin, BufRead, BufWriter, Write, Read};
use std::collections::HashSet;
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

    let mut d: HashSet<&str> = HashSet::new();
    let mut ans: Vec<&str> = Vec::new();

    buf.clear();
    stdin.read_to_string(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace();

    for _ in 1..=n {
        d.insert(buf.next().unwrap());
    }

    for _ in 0..m {
        let tmp = buf.next().unwrap();
        if d.contains(tmp) {
            ans.push(tmp);
        }
    }
    ans.sort_unstable();

    writeln!(stdout, "{}", ans.len()).unwrap();
    for str in ans {
        writeln!(stdout, "{}", str).unwrap();
    }
}