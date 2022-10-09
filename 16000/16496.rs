use std::io::{stdout, stdin, BufRead, BufWriter, Write};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut arr: Vec<&str> = buf.split_ascii_whitespace().collect();

    arr.sort_unstable_by(|a, b| (b.to_string() + a).cmp(&(a.to_string() + b)));

    if arr[0].parse::<u64>().unwrap() == 0 {
        writeln!(stdout, "0").unwrap();
    } else {
        for str in arr {
            write!(stdout, "{}", str).unwrap();
        }
        writeln!(stdout, "").unwrap();
    }
}