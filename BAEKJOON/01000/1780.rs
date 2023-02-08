use std::io::{stdout, stdin, BufWriter, Write, BufRead, Read};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut g: Vec<Vec<i8>> = vec![vec![]; n];
    buf.clear();
    stdin.read_to_string(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse);

    for i in 0..n {
        for _ in 0..n {
            g[i].push(buf.next().unwrap());
        }
    }
    let ans = rec(&g, n, 0, 0);
    writeln!(stdout, "{}", ans.0).unwrap();
    writeln!(stdout, "{}", ans.1).unwrap();
    writeln!(stdout, "{}", ans.2).unwrap();
}

fn rec(g: &Vec<Vec<i8>>, n: usize, r: usize, c: usize) -> (usize, usize, usize) {
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
        match g[r][c] {
            -1 => return (1, 0, 0),
            0 => return (0, 1, 0),
            1 => return (0, 0, 1),
            _ => (),
        }
    }
    let tmp = n / 3;
    let mut ans: Vec<usize> = vec![0;3];
    for i in 0..3 {
        for j in 0..3 {
            let tmp = rec(g, tmp, r + i * tmp, c + j * tmp);
            ans[0] += tmp.0;
            ans[1] += tmp.1;
            ans[2] += tmp.2;
        }
    }
    (ans[0], ans[1], ans[2])
}