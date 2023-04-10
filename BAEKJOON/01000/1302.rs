use std::io::{Result, stdout, stdin, BufWriter, Write, BufRead, BufReader};
use std::collections::HashMap;
fn main() -> Result<()> {
    let mut stdin_bufreader = BufReader::new(stdin());
    let mut stdout_bufwriter = BufWriter::new(stdout());

    let mut buf = String::new();
    stdin_bufreader.read_line(&mut buf)?;
    let mut hmap: HashMap<_, i32> = HashMap::new();

    for _ in 0..buf.trim().parse::<i32>().unwrap() {
        let mut buf = String::new();
        stdin_bufreader.read_line(&mut buf)?;
        if let Some(val) = hmap.get_mut(&buf) {
            *val += 1;
        }
        else {
            hmap.insert(buf, 1);
        }
    }
    writeln!(stdout_bufwriter, "{}", hmap.iter().max_by(|&x, &y| x.1.cmp(y.1).then(y.0.cmp(x.0))).unwrap().0)?;
    Ok(())
}
