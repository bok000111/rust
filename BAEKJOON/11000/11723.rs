use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    let mut set: i32 = 0;
    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut tmp = buf.split_ascii_whitespace();
        match tmp.next().unwrap() {
            "add" => set |= 1 << tmp.next().unwrap().parse::<i32>().unwrap(),
            "remove" => set &= !(1 << tmp.next().unwrap().parse::<i32>().unwrap()),
            "check" => {
                match set & (1 << tmp.next().unwrap().parse::<i32>().unwrap()) {
                    0 => writeln!(stdout, "0").unwrap(),
                    _ => writeln!(stdout, "1").unwrap(),
                }
            },
            "toggle" => set ^= 1 << tmp.next().unwrap().parse::<i32>().unwrap(),
            "all" => set = (1 << 21) - 1,
            "empty" => set = 0,
            _ => (),
        }
    }
}