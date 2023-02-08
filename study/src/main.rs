use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Result, Write};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let mut stdin_bufreader = BufReader::new(stdin());
    let mut stdout_bufwriter = BufWriter::new(stdout());

    let mut buf = String::new();
    stdin_bufreader.read_line(&mut buf)?;
    let mut a: VecDeque<i32> = buf.split_ascii_whitespace().flat_map(|x| x.parse()).collect();
    //let mut b: VecDeque<i32> = VecDeque::new();

    for i in a.iter() {
        writeln!(stdout_bufwriter, "{}", i).unwrap();
    }
    Ok(())
}
