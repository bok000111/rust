use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let len = buf.trim_end().len() as i32;
    let mut str: Vec<i32> = vec![0; 26];
    buf.trim_end().bytes().for_each(|x| str[x as usize - 65] += 1);
    let mut cnt = 0;
    for _ in 1..n {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let tmp = buf.trim_end().len() as i32;
        let mut v: Vec<i32> = vec![0; 26];
        buf.trim_end().bytes().for_each(|x| v[x as usize - 65] += 1);
        let diff: i32 = (0..26).map(|x| (str[x] - v[x]).abs()).sum();
        if (len - tmp).abs() <= 1 && diff <= 2 {
            cnt += 1;
        }
    }
    writeln!(stdout, "{}", cnt).unwrap();
}