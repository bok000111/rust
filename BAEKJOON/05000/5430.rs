use std::{io::{stdout, stdin, BufRead, BufWriter, Write}, collections::VecDeque};
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
        for _ in 0..3 {
            stdin.read_line(&mut buf).unwrap();
        }
        let mut tmp = buf.split_ascii_whitespace();
        let input = tmp.next().unwrap().chars();
        let _n: usize = tmp.next().unwrap().parse().unwrap();
        let mut arr: VecDeque<i32> = tmp.next().unwrap().trim_matches(|x| x == '[' || x == ']').split(',').flat_map(str::parse::<i32>).collect();

        let mut flag = true;
        let mut flag2 = true;
        for c in input {
            match c {
                'R' => flag ^= true,
                'D' => {
                    match flag {
                        true => match arr.pop_front() {
                            None => {
                                flag2 = false;
                                break;
                            },
                            _ => (),
                        }
                        false => match arr.pop_back() {
                            None => {
                                flag2 = false;
                                break;
                            },
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }
        match flag2 {
            true => {
                write!(stdout, "[").unwrap();
                let l = arr.len();
                if flag {
                    for i in 0..l {
                        write!(stdout, "{}", arr[i]).unwrap();
                        if i != l - 1 {
                            write!(stdout, ",").unwrap();
                        }
                    }
                } else {
                    for i in (0..l).rev() {
                        write!(stdout, "{}", arr[i]).unwrap();
                        if i != 0 {
                            write!(stdout, ",").unwrap();
                        }
                    }
                }
                writeln!(stdout, "]").unwrap();
            },
            false => writeln!(stdout, "error").unwrap(),
        }
    }
}