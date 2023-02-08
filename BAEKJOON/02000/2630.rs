use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut g: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        g.push(buf.split_ascii_whitespace().flat_map(str::parse).collect());
    }
    let ans = rec(&g, n, 0, 0);
    writeln!(stdout, "{}", ans.0).unwrap();
    writeln!(stdout, "{}", ans.1).unwrap();
}

fn rec(g: &Vec<Vec<usize>>, n: usize, r: usize, c: usize) -> (usize, usize) {
    let mut flag = true;
    for i in r..r + n {
        for j in c..c + n {
            if g[i][j] != g[r][c] {
                flag = false;
                break;
            }
        }
        if !flag {
            break;
        }
    }
    if flag {
        if g[r][c] == 0 {
            return (1, 0);
        } else {
            return (0, 1);
        }
    }
    let ans1 = rec(g, n / 2, r, c);
    let ans2 = rec(g, n / 2, r, c + n / 2);
    let ans3 = rec(g, n / 2, r + n / 2, c);
    let ans4 = rec(g, n / 2, r + n / 2, c + n / 2);

    (ans1.0 + ans2.0 + ans3.0 + ans4.0, ans1.1 + ans2.1 + ans3.1 + ans4.1)
}