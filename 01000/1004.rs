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
        stdin.read_line(&mut buf).unwrap();
        let s: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let n: usize = buf.trim_end().parse().unwrap();
        let mut v: Vec<Vec<i32>> = vec![];
        let mut cnt = 0;
        
        for _ in 0..n {
            {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();
            }
            v.push(buf.split_ascii_whitespace().flat_map(str::parse::<i32>).collect::<Vec<i32>>())
        }

        for crd in v {
            let dis1 = (s[0] - crd[0]).pow(2) + (s[1] - crd[1]).pow(2);
            let dis2 = (s[2] - crd[0]).pow(2) + (s[3] - crd[1]).pow(2);
            cnt += ((dis1 < crd[2].pow(2)) ^ (dis2 < crd[2].pow(2))) as i32;
        }
        writeln!(stdout, "{}", cnt).unwrap();
    }
}
