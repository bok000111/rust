use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    let mut dp: Vec<usize> = vec![0; 12];

    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 4;

    for i in 4..=11 {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }

    buf.clear();
    for _ in 0..n {
        stdin.read_line(&mut buf).unwrap();
    }
    let buf = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    buf.for_each(|x| writeln!(stdout, "{}", dp[x]).unwrap());
}