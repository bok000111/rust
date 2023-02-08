use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse);
    let n: i32 = buf.next().unwrap();
    let m: i32 = buf.next().unwrap();

    writeln!(stdout, "{}", std::cmp::min(count_2(n) - (count_2(m) + count_2(n - m)), count_5(n) - (count_5(m) + count_5(n - m)))).unwrap();
}

fn count_5(n: i32) -> i32 {
    let mut cnt = 0;
    let mut n = n;
    while n > 0 {
        n /= 5;
        cnt += n;
    }
    cnt
}

fn count_2(n: i32) -> i32 {
    let mut cnt = 0;
    let mut n = n;
    while n > 0 {
        n /= 2;
        cnt += n;
    }
    cnt
}