use std::io::{stdout, stdin, BufWriter, Write, BufRead};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut tmp = buf.trim_end().chars();
    let mut stack = vec![];
    loop {
        match tmp.next() {
            Some('<') => {
                while let Some(a) = stack.pop() {
                    write!(stdout, "{}", a).unwrap();
                }
                write!(stdout, "<", ).unwrap();
                while let Some(c) = tmp.next() {
                    match c {
                        '>' => {
                            write!(stdout, ">").unwrap();
                            break;
                        },
                        c => write!(stdout, "{}", c).unwrap(),
                    }
                }
            },
            Some(' ') => {
                while let Some(a) = stack.pop() {
                    write!(stdout, "{}", a).unwrap();
                }
                write!(stdout, " ", ).unwrap();
            },
            Some(c) => stack.push(c),
            None => {
                while let Some(a) = stack.pop() {
                    write!(stdout, "{}", a).unwrap();
                }
                break;
            },
        }
    }
    writeln!(stdout, "").unwrap();
}