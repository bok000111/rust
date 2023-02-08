use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n + 2];
    dp[2][0] = 1;
    dp[2][1] = 1;

    for i in 3..=n {
        dp[i][0] = (dp[i - 1][1] + dp[i - 1][2]) % 1000000007;
        dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % 1000000007;
        dp[i][2] = (dp[i - 1][0] + dp[i - 1][1]) % 1000000007;
    }
    writeln!(stdout, "{}", dp[n][0]).unwrap();
}
