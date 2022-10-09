use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse);
    let n: u32 = buf.next().unwrap();
    let r: u32 = buf.next().unwrap();
    let c: u32 = buf.next().unwrap();

    writeln!(stdout, "{}", rec(n, r, c, 0)).unwrap();
}

fn rec(n: u32, r: u32, c: u32, s: u32) -> u32 {
    if n == 1 {
        return s + 2 * r + c;
    }
    let l = 2_u32.pow(n - 1);
    if r < l {
        if c < l {
            return rec(n - 1, r, c, s);
        } else {
            return rec(n - 1, r, c - l, s + l * l);
        }
    } else {
        if c < l {
            return rec(n - 1, r - l, c, s + 2 * l * l);
        } else {
            return rec(n - 1, r - l, c - l, s + 3 * l * l);
        }
    }
}
