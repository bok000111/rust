use std::{io::{stdout, stdin, BufRead, BufWriter, Write}, collections::HashMap};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut map: HashMap<i64, usize> = HashMap::new();

    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let tmp: i64 = buf.trim_end().parse().unwrap();

        match map.get(&tmp) {
            Some(&a) => map.insert(tmp, a + 1),
            None => map.insert(tmp, 1),
        };
    }
    writeln!(stdout, "{}", map.iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0))).unwrap().0).unwrap();
}