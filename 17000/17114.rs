use std::{io::{stdout, stdin, BufWriter, Write, BufRead}, collections::VecDeque};
fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let nm: Vec<usize> = buf.split_ascii_whitespace().flat_map(str::parse).collect();
    let mut g: Vec<_> =  vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![-1; nm[0]]; nm[1]]; nm[2]]; nm[3]]; nm[4]]; nm[5]]; nm[6]]; nm[7]]; nm[8]]; nm[9]]; nm[10]];
    let mut q: VecDeque<(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)> = VecDeque::new();
    let mut tomato = 0;
    let mut cnt = 0;
    let mut day = 0;

    buf.clear();
    for _ in 0..nm.iter().skip(1).product() {
        stdin.read_line(&mut buf).unwrap();
    }
    let mut buf = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    for i0 in 0..nm[10] {
        for i1 in 0..nm[9] {
            for i2 in 0..nm[8] {
                for i3 in 0..nm[7] {
                    for i4 in 0..nm[6] {
                        for i5 in 0..nm[5] {
                            for i6 in 0..nm[4] {
                                for i7 in 0..nm[3] {
                                    for i8 in 0..nm[2] {
                                        for i9 in 0..nm[1] {
                                            for i10 in 0..nm[0] {
                                                let tmp = buf.next().unwrap();
                                                if tmp == 1 {
                                                    tomato += 1;
                                                    cnt += 1;
                                                    q.push_back((i0, i1, i2, i3, i4, i5, i6, i7, i8, i9, i10));
                                                    g[i0][i1][i2][i3][i4][i5][i6][i7][i8][i9][i10] = tmp;
                                                } else if tmp == 0 {
                                                    tomato += 1;
                                                    g[i0][i1][i2][i3][i4][i5][i6][i7][i8][i9][i10] = tmp;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let da = [1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let db = [0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let dc = [0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let dd = [0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let de = [0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let df = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let dg = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0];
    let dh = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0];
    let di = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0];
    let dj = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0];
    let dk = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1];

    while let Some(now) = q.pop_front() {
        for i in 0..22 {
            let n_a = now.0 as i32 + da[i];
            let n_b = now.1 as i32 + db[i];
            let n_c = now.2 as i32 + dc[i];
            let n_d = now.3 as i32 + dd[i];
            let n_e = now.4 as i32 + de[i];
            let n_f = now.5 as i32 + df[i];
            let n_g = now.6 as i32 + dg[i];
            let n_h = now.7 as i32 + dh[i];
            let n_i = now.8 as i32 + di[i];
            let n_j = now.9 as i32 + dj[i];
            let n_k = now.10 as i32 + dk[i];

            if 0 <= n_a && n_a < nm[10] as i32 && 0 <= n_b && n_b < nm[9] as i32 && 0 <= n_c && n_c < nm[8] as i32 && 0 <= n_d && n_d < nm[7] as i32 {
                if 0 <= n_e && n_e < nm[6] as i32 && 0 <= n_f && n_f < nm[5] as i32 && 0 <= n_g && n_g < nm[4] as i32 && 0 <= n_h && n_h < nm[3] as i32 {
                    if 0 <= n_i && n_i < nm[2] as i32 && 0 <= n_j && n_j < nm[1] as i32 && 0 <= n_k && n_k < nm[0] as i32 {
                        if g[n_a as usize][n_b as usize][n_c as usize][n_d as usize][n_e as usize][n_f as usize][n_g as usize][n_h as usize][n_i as usize][n_j as usize][n_k as usize] == 0 {
                            q.push_back((n_a as usize, n_b as usize, n_c as usize, n_d as usize, n_e as usize, n_f as usize, n_g as usize, n_h as usize, n_i as usize, n_j as usize, n_k as usize));
                            cnt += 1;
                            g[n_a as usize][n_b as usize][n_c as usize][n_d as usize][n_e as usize][n_f as usize][n_g as usize][n_h as usize][n_i as usize][n_j as usize][n_k as usize] = g[now.0][now.1][now.2][now.3][now.4][now.5][now.6][now.7][now.8][now.9][now.10] + 1;
                            if day < g[now.0][now.1][now.2][now.3][now.4][now.5][now.6][now.7][now.8][now.9][now.10] {
                                day = g[now.0][now.1][now.2][now.3][now.4][now.5][now.6][now.7][now.8][now.9][now.10];
                            }
                        }
                    }
                }
            }
        }
    }

    if cnt != tomato {
        writeln!(stdout, "-1").unwrap();
    } else {
        writeln!(stdout, "{}", day).unwrap();
    }
}