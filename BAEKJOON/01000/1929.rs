use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = tmp.next().unwrap();
    let m = tmp.next().unwrap();

    let mut v: Vec<bool> = (0..m + 1).map(|x| x > 1).collect();
    (2..=m).for_each(|i| if v[i] {(i..=m / i).for_each(|j| v[i * j] =  false)});

    (n..=m).filter(|&i| v[i]).for_each(|i| writeln!(stdout, "{}", i).unwrap());
}