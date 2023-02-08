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
    let m: usize = buf.trim_end().parse().unwrap();

    let prime = era(n, m);
    if prime.len() == 0 {
        writeln!(stdout, "-1").unwrap();
    } else {
        writeln!(stdout, "{}", prime.iter().sum::<usize>()).unwrap();
        writeln!(stdout, "{}", prime[0]).unwrap();
    }
}

fn era(n: usize, m: usize) -> Vec<usize> {
    let mut v: Vec<bool> = vec![true;m + 1];
    v[0] = false;
    v[1] = false;
    (2..=m).for_each(|i| if v[i] {(i..=m / i).for_each(|j| v[i * j] =  false)});
    (n..=m).filter(|&x| v[x]).collect()
}