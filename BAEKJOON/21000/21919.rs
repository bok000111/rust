use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let v = era(1000000);
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let ans = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut ans: Vec<usize> = ans.filter(|&x| v[x]).collect();
    if ans.len() == 0 {
        writeln!(stdout, "-1").unwrap();
    } else {
        ans.sort_unstable();
        ans.dedup();
        writeln!(stdout, "{}", ans.iter().product::<usize>()).unwrap();
    }
}

fn era(n: usize) -> Vec<bool> {
    let mut v: Vec<bool> = vec![true;n + 1];
    (2..=n).for_each(|i| if v[i] {(i..=n / i).for_each(|j| v[i * j] =  false)});
    v
}