use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut sum: i128 = 0;
    let mut xor: i32 = 0;

    for _ in 0..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
        match tmp.next().unwrap() {
            1 => {
                let a = tmp.next().unwrap();
                sum += a as i128;
                xor ^= a;
            },
            2 => {
                let a = tmp.next().unwrap();
                sum -= a as i128;
                xor ^= a;
            },
            3 => writeln!(stdout, "{}", sum).unwrap(),
            4 => writeln!(stdout, "{}", xor).unwrap(),
            _ => (),
        }
    }
}