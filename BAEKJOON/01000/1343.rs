use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.trim_end().chars();
    let mut stack: Vec<char> = vec![];
    let mut ans = vec![];
    while let Some(c) = buf.next() {
        match c {
            'X' => stack.push(c),
            '.' => {
                if stack.len() % 2 == 1 {
                    ans.clear();
                    ans.push('-');
                    ans.push('1');
                    break;
                } else {
                    for _ in 0..stack.len() / 4 {
                        ans.push('A');
                        ans.push('A');
                        ans.push('A');
                        ans.push('A');
                    }
                    for _ in 0..stack.len() % 4 {
                        ans.push('B');
                    }
                    ans.push('.');
                    stack.clear();
                }
            },
            _ => (),
        }
    }
    if stack.len() % 2 == 1 {
        ans.clear();
        ans.push('-');
        ans.push('1');
    } else {
        for _ in 0..stack.len() / 4 {
            ans.push('A');
            ans.push('A');
            ans.push('A');
            ans.push('A');
        }
        for _ in 0..stack.len() % 4 {
            ans.push('B');
        }
    }
    for i in 0..ans.len() {
        write!(stdout, "{}", ans[i]).unwrap();
    }
    writeln!(stdout, "").unwrap();
}