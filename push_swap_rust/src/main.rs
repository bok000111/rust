#![deny(warnings)]

//use std::io::{stdout, BufWriter, Result, Write};
use std::io::Result;
use std::env;
use std::collections::{VecDeque};
use std::cmp::Ordering;
mod ft_stack;

fn main() -> Result<()> {
    //let mut stdout_bufwriter = BufWriter::new(stdout());
    //let mut a =  FtStack{dq: env::args().skip(1).map(|x| x.parse::<i32>().unwrap()).collect(), name: 'a'};
    let tmp = env::args().skip(1).next().unwrap();
    let mut a = ft_stack::FtStack{dq: tmp.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect(), name: 'a'};
    let mut b =  ft_stack::FtStack{dq: VecDeque::new(), name: 'b'};
    let mut cmd = VecDeque::new();
    let size = a.dq.len();
    ft_stack::FtStack::sort(&mut a, &mut b, size, Ordering::Greater, &mut cmd);
    for i in cmd {
        ft_stack::FtStack::prt_cmd(i);
    }
    Ok(())
}
