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
        let v: Vec<f64> = buf.split_ascii_whitespace().flat_map(str::parse).collect();

        let dis = ((v[0] - v[3]).powf(2_f64) + (v[1] - v[4]).powf(2_f64)).sqrt();

        if dis == 0_f64 && v[2] == v[5] {
            writeln!(stdout, "-1").unwrap();
        } else if (v[2] - v[5]).abs() < dis && dis < v[2] + v[5] {
            writeln!(stdout, "2").unwrap();
        } else if dis == v[2] + v[5] || dis == (v[2] - v[5]).abs() {
            writeln!(stdout, "1").unwrap();
        } else {
            writeln!(stdout, "0").unwrap();
        }
    }
}
