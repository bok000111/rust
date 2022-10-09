use std::io::{stdout, BufWriter, Write};
fn main() {
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut v: Vec<bool> = vec![true;10001];
    
    for i in 1..=10000 {
        if v[i] {
            let mut tmp = i + i.to_string().bytes().map(|x| (x - 48) as usize).sum::<usize>();
            while tmp <= 10000 && v[tmp] {
                v[tmp] = false;
                tmp = tmp + tmp.to_string().bytes().map(|x| (x - 48) as usize).sum::<usize>();
            }
        }
    }
    (1..=10000).filter(|&i| v[i]).for_each(|i| writeln!(stdout, "{}", i).unwrap());
}