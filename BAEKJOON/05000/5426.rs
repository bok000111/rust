use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.trim().chars();
        let len = i_sqrt(buf.len() - 1);
        let mut arr: Vec<Vec<char>> = vec![vec!['0'; len]; len];
        for j in 0..len {
            for i in (0..len).rev() {
                arr[i][j] = tmp.next().unwrap();
            }
        }
        for i in 0..len {
            for j in 0..len {
                write!(stdout, "{}", arr[i][j]).unwrap();
            }
        }
        writeln!(stdout, "").unwrap();
    }
}

fn i_sqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}