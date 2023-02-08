use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t = buf.trim_end().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let arr: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).skip(1).collect();
        let a = arr.iter().enumerate().filter(|&(i, &val)| i % 2 == 0 && val == 1).count() as i32;
        let b = arr.iter().enumerate().filter(|&(i, &val)| i % 2 == 1 && val == 1).count() as i32;
        if arr.len() % 2 == 1 || (a - b).abs() <= 1 {
            writeln!(stdout, "YES").unwrap();
        } else {
            writeln!(stdout, "NO").unwrap();
        }
    }
}