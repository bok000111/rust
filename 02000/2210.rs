use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::HashSet};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    let mut g: Vec<Vec<usize>> = vec![];

    for _ in 0..5 {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        g.push(buf.split_ascii_whitespace().flat_map(str::parse).collect())
    }
    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..5 {
        for j in 0..5 {
            back(&g, &mut set, 0, i, j, 0);
        }
    }
    writeln!(stdout, "{}", set.len()).unwrap();
}

fn back(g: &Vec<Vec<usize>>, set: &mut HashSet<usize>, num: usize, r: usize, c: usize, d: usize) {
    if d == 6 {
        set.insert(num);
    } else {
        if r < 4 {
            back(g, set, num * 10 + g[r + 1][c], r + 1, c, d + 1);
        }
        if r > 0 {
            back(g, set, num * 10 + g[r - 1][c], r - 1, c, d + 1);
        }
        if c < 4 {
            back(g, set, num * 10 + g[r][c + 1], r, c + 1, d + 1);
        }
        if c > 0 {
            back(g, set, num * 10 + g[r][c - 1], r, c - 1, d + 1);
        }
    }
}