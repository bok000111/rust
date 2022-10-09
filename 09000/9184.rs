use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; 21]; 21]; 21];

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let v: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
        if v[0] == -1 && v[1] == -1 && v[2] == -1 {
            break;
        }
        writeln!(stdout, "w({}, {}, {}) = {}", v[0], v[1], v[2], w(v[0], v[1], v[2], &mut dp)).unwrap();
    }
}

fn w(a: i32, b: i32, c: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    } else if a > 20 || b > 20 || c > 20 {
        return w(20, 20, 20, dp);
    }
    
    if dp[a as usize][b as usize][c as usize] == -1 {
        if a < b && b < c {
            dp[a as usize][b as usize][c as usize] = w(a, b, c - 1, dp) + w(a, b - 1, c - 1, dp) - w(a, b - 1, c, dp);
        } else {
            dp[a as usize][b as usize][c as usize] = w(a - 1, b, c, dp) + w(a - 1, b - 1, c, dp) + w(a - 1, b, c - 1, dp) - w(a - 1, b - 1, c - 1, dp);
        }
    }
    dp[a as usize][b as usize][c as usize]
}