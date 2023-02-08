use std::{io::{stdout, stdin, BufRead, BufWriter, Write}, collections::BinaryHeap};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let t: usize = buf.trim_end().parse().unwrap();
    let mut heap = BinaryHeap::new();

    for _ in 0..t {
        buf.clear();
        {
            stdin.read_line(&mut buf).unwrap();
        }
        let n: usize = buf.trim_end().parse().unwrap();
        match n {
            0 => writeln!(stdout, "{}", heap.pop().unwrap_or(0)).unwrap(),
            n => heap.push(n),
        }
    }
}