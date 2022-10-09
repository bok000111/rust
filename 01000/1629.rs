use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse);
    let a: i128 = buf.next().unwrap();
    let b: i128 = buf.next().unwrap();
    let c: i128 = buf.next().unwrap();

    writeln!(stdout, "{}", modpow(a, b, c)).unwrap();
}

fn modpow(mut base: i128, mut pow: i128, i_mod: i128) -> i128
{
    let mut res: i128 = 1;
    loop
    {
        if pow & 1 == 1
        {
            res = res * base % i_mod;
        }
        base = base * base % i_mod;
        pow >>= 1;
        if pow == 0
        {
            return res;
        }
    }
}