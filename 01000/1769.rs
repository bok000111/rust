use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let num = buf.trim_end().to_string();

    let ans = rec(&num, 0);
    if ans.1 {
        writeln!(stdout, "{}", ans.0).unwrap();
        writeln!(stdout, "YES").unwrap();
    } else {
        writeln!(stdout, "{}", ans.0).unwrap();
        writeln!(stdout, "NO").unwrap();
    }
}

fn rec(num: &String, d: usize) -> (usize, bool) {
    if num.len() == 1 {
        return (d, num.parse::<usize>().unwrap() % 3 == 0);
    }
    let tmp = num.bytes().map(|x| x as usize - 48).sum::<usize>().to_string();
    rec(&tmp, d + 1)
}