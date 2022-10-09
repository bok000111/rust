use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut v: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        v.push(buf.split_ascii_whitespace().flat_map(str::parse).collect());
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n + 1];

    dp[1] = v[1].clone();
    for i in 2..=n {
        dp[i][0] = std::cmp::min(dp[i - 1][1] + v[i][0], dp[i - 1][2] + v[i][0]);
        dp[i][1] = std::cmp::min(dp[i - 1][0] + v[i][1], dp[i - 1][2] + v[i][1]);
        dp[i][2] = std::cmp::min(dp[i - 1][0] + v[i][2], dp[i - 1][1] + v[i][2]);
    }
    writeln!(stdout, "{:?}", std::cmp::min(dp[n][0], std::cmp::min(dp[n][1], dp[n][2]))).unwrap()
}
