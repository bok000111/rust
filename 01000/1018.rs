use std::io::{stdout, stdin, BufRead, BufWriter, Write};

fn main()
{
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n: usize = input.next().unwrap();
    let m: usize = input.next().unwrap();

    buf.clear();
    for _i in 0..n
    {
        stdin.read_line(&mut buf).unwrap();
    }
    let v: Vec<Vec<char>> = buf.split_ascii_whitespace().map(|x| x.chars().collect::<Vec<char>>()).collect();
    let mut min = n * m;

    for i in 0..=n - 8
    {
        for j in 0..=m - 8
        {
            let mut w = 0;
            let mut b = 0;
            for k in i..i + 8
            {
                for l in j..j + 8
                {
                    if (k + l) % 2 == 0
                    {
                        match v[k][l]
                        {
                            'B' => w = w + 1,
                            _ => b = b + 1
                        }
                    }
                    else
                    {
                        match v[k][l]
                        {
                            'W' => w = w + 1,
                            _ => b = b + 1
                        }
                    }
                }
            }
            min = std::cmp::min(min, std::cmp::min(w, b));
        }
    }
    writeln!(stdout, "{}", min).unwrap();
}