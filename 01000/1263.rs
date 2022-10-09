use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut v: Vec<(i32, i32)> = vec![];
    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse);
        v.push((tmp.next().unwrap(), tmp.next().unwrap()));
    }
    v.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut time = 1000001;
    for (t, s) in v {
        time = std::cmp::min(time - t, s - t);
    }
    if time < 0 {
        writeln!(stdout, "-1").unwrap();
    } else {
        writeln!(stdout, "{}", time).unwrap();
    }
}
