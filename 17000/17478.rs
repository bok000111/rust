use std::io::{stdout, stdin, BufWriter, Write, BufRead, StdoutLock};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim_end().parse().unwrap();

    writeln!(stdout, "어느 한 컴퓨터공학과 학생이 유명한 교수님을 찾아가 물었다.").unwrap();
    rec(n + 1, 0, &mut stdout);
}

fn rec(n: i32, d: i32, stdout: &mut BufWriter<StdoutLock>) {
    if d == n {
        return;
    }
    for _ in 0..d {
        write!(stdout, "____").unwrap();
    }
    writeln!(stdout, "\"재귀함수가 뭔가요?\"").unwrap();
    if d == n - 1 {
        for _ in 0..d {
            write!(stdout, "____").unwrap();
        }
        writeln!(stdout, "\"재귀함수는 자기 자신을 호출하는 함수라네\"").unwrap();
    } else {
        for _ in 0..d {
            write!(stdout, "____").unwrap();
        }
        writeln!(stdout, "\"잘 들어보게. 옛날옛날 한 산 꼭대기에 이세상 모든 지식을 통달한 선인이 있었어.").unwrap();
        for _ in 0..d {
            write!(stdout, "____").unwrap();
        }
        writeln!(stdout, "마을 사람들은 모두 그 선인에게 수많은 질문을 했고, 모두 지혜롭게 대답해 주었지.").unwrap();
        for _ in 0..d {
            write!(stdout, "____").unwrap();
        }
        writeln!(stdout, "그의 답은 대부분 옳았다고 하네. 그런데 어느 날, 그 선인에게 한 선비가 찾아와서 물었어.\"").unwrap();
        rec(n, d + 1, stdout);
    }
    for _ in 0..d {
        write!(stdout, "____").unwrap();
    }
    writeln!(stdout, "라고 답변하였지.").unwrap();
}