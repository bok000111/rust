use std::io::{stdout, stdin, BufRead, BufWriter, Write};
use std::collections::HashMap;
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

    let mut h_map: HashMap<usize, &str> = HashMap::new();
    let mut h_map2: HashMap<&str, usize> = HashMap::new();

    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf = buf.split_ascii_whitespace();

    for i in 1..=n {
        let name = buf.next().unwrap();
        h_map.insert(i, name);
        h_map2.insert(name, i);
    }

    let mut buf = String::new();
    for _i in 0..m {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf = buf.split_ascii_whitespace();
    for _ in 0..m {
        let tmp = buf.next().unwrap();
        match tmp.chars().any(|c| c.is_alphabetic()) {
            true => writeln!(stdout, "{}", h_map2[tmp]).unwrap(),
            false => writeln!(stdout, "{}", h_map[&tmp.parse::<usize>().unwrap()]).unwrap(),
        }
    }

}