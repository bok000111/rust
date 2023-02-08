use std::io::{stdout, stdin, BufWriter, Write, BufRead};
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
    let buf = buf.split_ascii_whitespace();

    writeln!(stdout, "{}", buf.filter(|x| is_group(x)).count()).unwrap();
}

fn is_group(a: &str) -> bool {
    let a: Vec<usize> = a.bytes().map(|x| x as usize - 97).collect();
    let mut arr = [0; 26];
    arr[a[0]] += 1;
    for i in 1..a.len() {
        if a[i - 1] != a[i] && arr[a[i]] != 0 {
            return false;
        }
        arr[a[i]] += 1;
    }
    true
}