use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut v = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let w = v.next().unwrap();
    let h = v.next().unwrap();
    let x = v.next().unwrap();
    let y = v.next().unwrap();
    let n = v.next().unwrap();
    let mut cnt = 0;
    for _ in 0..n {
        {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
        }
        let p: Vec<i32> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
        if p[0] < x {
            if (p[0] - x).pow(2) + (p[1] - (y + h / 2)).pow(2) <= (h / 2).pow(2) {
                cnt += 1;
            }
        } else if p[0] > x + w {
            if (p[0] - (x + w)).pow(2) + (p[1] - (y + h / 2)).pow(2) <= (h / 2).pow(2) {
                cnt += 1;
            }
        } else if y <= p[1] && p[1] <= y + h {
            cnt += 1;
        }
    }

    writeln!(stdout, "{}", cnt).unwrap();
}