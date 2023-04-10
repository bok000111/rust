use std::io::{Result, stdout, stdin, BufWriter, Write, BufRead, BufReader};
use std::collections::BTreeSet;
fn main() -> Result<()> {
    let mut stdin_bufreader = BufReader::new(stdin());
    let mut stdout_bufwriter = BufWriter::new(stdout());

    let mut buf = String::new();
    stdin_bufreader.read_line(&mut buf)?;
    let buf = buf.trim();

    let mut btset = BTreeSet::new();
    for x in 0..buf.len() {
        btset.insert(buf[x..].to_string());
    }
    for x in btset.iter() {
        writeln!(stdout_bufwriter, "{}", x)?;
    }
    Ok(())
}