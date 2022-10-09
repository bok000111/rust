use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut arr = [0; 10];
    buf.trim_end().bytes().for_each(|x| arr[(x - 48)as usize] += 1);

    if arr[0] == 0 || arr.iter().enumerate().map(|(i, &n)| i as i32 * n).sum::<i32>() % 3 != 0 {
        writeln!(stdout, "-1").unwrap();
    } else {
        for i in (0..10).rev() {
            for _ in 0..arr[i] {
                write!(stdout, "{}", i).unwrap();
            }
        }
        writeln!(stdout, "").unwrap();
    }
}