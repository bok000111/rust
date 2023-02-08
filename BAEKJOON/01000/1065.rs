use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim_end().parse().unwrap();

    writeln!(stdout, "{}", (1..=n).filter(|&x| is_arithmetic(x)).count()).unwrap();
}

fn is_arithmetic(n: i32) -> bool {

    if n < 100 {
        return true;
    }
    let n: Vec<i32> = n.to_string().bytes().map(|c| c as i32).collect();

    let dif = n[1] - n[0];

    for i in 2..(n.len()) {
        if n[i] - n[i - 1] != dif {
            return false;
        }
    }
    true
}