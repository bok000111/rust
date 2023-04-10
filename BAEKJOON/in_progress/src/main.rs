use std::io::{Result, stdout, stdin, BufWriter, Write, BufRead, BufReader};
fn main() -> Result<()> {
    let mut stdin_bufreader = BufReader::new(stdin());
    let mut stdout_bufwriter = BufWriter::new(stdout());

    let mut buf = String::new();
    stdin_bufreader.read_line(&mut buf)?;
    writeln!(stdout_bufwriter, "{}", buf)?;
    Ok(())
}