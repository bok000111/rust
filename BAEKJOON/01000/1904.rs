use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    let mut dp = vec![0_usize; n + 2];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = (dp[i - 1] + dp[i - 2]) % 15746;
    }
    writeln!(stdout, "{}", dp[n]).unwrap();
}