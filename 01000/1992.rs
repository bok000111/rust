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
        g.push(buf.trim_end().bytes().map(|x| x as usize - 48).collect());
    }
    let ans = rec(&g, n, 0, 0);
    writeln!(stdout, "{}", ans).unwrap();
}

fn rec(g: &Vec<Vec<usize>>, n: usize, r: usize, c: usize) -> String {
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
            return "0".to_string();
        } else {
            return "1".to_string();
        }
    }
    format!("({}{}{}{})", rec(g, n / 2, r, c), rec(g, n / 2, r, c + n / 2), rec(g, n / 2, r + n / 2, c), rec(g, n / 2, r + n / 2, c + n / 2))
}