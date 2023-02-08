use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut n: i32 = buf.trim_end().parse().unwrap();
    if n % 5 == 0 {
        writeln!(stdout, "{}", n / 5).unwrap();
    } else {
        let mut ans = 0;
        while n > 0 {
            n -= 3;
            ans += 1;
            if n % 5 == 0 {
                writeln!(stdout, "{}", ans + n / 5).unwrap();
                break;
            }
            match n {
                0 => writeln!(stdout, "{}", ans).unwrap(),
                1 | 2 => writeln!(stdout, "-1").unwrap(),
                _ => (),
            }
        }
    }
}