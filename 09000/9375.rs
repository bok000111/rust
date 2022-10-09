use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::HashMap};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let n: usize = buf.trim_end().parse().unwrap();

        buf.clear();
        for _ in 0..n {
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.split_ascii_whitespace().skip(1).step_by(2);
        let mut map = HashMap::new();
        while let Some(str) = tmp.next() {
            if let Some(a) = map.get_mut(str) {
                *a += 1;
            } else {
                map.insert(str, 2);
            }
        }
        let mut ans = 1;
        map.iter().for_each(|x| ans *= *x.1);
        writeln!(stdout, "{}", ans - 1).unwrap();
    }
}