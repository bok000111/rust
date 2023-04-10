use std::io::{Result, stdout, stdin, BufWriter, Write, BufRead, BufReader};
fn main() -> Result<()> {
    let mut stdin_bufreader = BufReader::new(stdin());
    let mut stdout_bufwriter = BufWriter::new(stdout());

    let mut buf = String::new();
    stdin_bufreader.read_line(&mut buf)?;
    let buf = buf.trim();

    writeln!(stdout_bufwriter, "{}", calc(buf))?;
    Ok(())
}

fn calc(str: &str) -> i32 {
    let c = str.chars().nth(0).unwrap();
    for x in str.chars() {
        if x != c {
            let tmp = str.trim_matches(str.chars().nth(0).unwrap());
            return calc(tmp) + 1;
        }
    }
    0
}
