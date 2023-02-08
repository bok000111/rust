use std::io::{stdout, stdin, BufWriter, Write, Read};
use std::collections::HashMap;
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace();
    let n: usize = buf.next().unwrap().parse().unwrap();
    let m: usize = buf.next().unwrap().parse().unwrap();

    let mut map: HashMap<&str, &str> = HashMap::new();

    for _ in 0..n {
        map.insert(buf.next().unwrap(), buf.next().unwrap());
    }

    for _ in 0..m {
        writeln!(stdout, "{}", map[buf.next().unwrap()]).unwrap();
    }
}