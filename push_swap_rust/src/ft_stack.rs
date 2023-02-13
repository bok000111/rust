#![deny(warnings)]

use std::collections::{VecDeque};
use std::cmp::Ordering;

pub struct FtStack {
    pub dq: VecDeque<i32>,
    pub name: char,
}

pub enum Cmd {
    SA,
    SB,
    SS,
    PA,
    PB,
    RA,
    RB,
    RR,
    RRA,
    RRB,
    RRR,
}

impl PartialEq for Cmd {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl FtStack {
    pub fn sort(a: &mut FtStack, b: &mut FtStack, size: usize, order: Ordering, cmd: &mut VecDeque<Cmd>) {
        if size < 3 {
            FtStack::sort_small(a, size, order, cmd)
        } else {
            FtStack::sort_push(a, b, size / 3, order, cmd);
            for _ in 0..(size / 3) {
                b.r(cmd);
            }
            FtStack::sort_push(a, b, size / 3, order.reverse(), cmd);
            FtStack::sort(a, b, size - 2 * (size / 3), order, cmd);
            for _ in 0..(size - 2 * (size / 3)) {
                a.r(cmd);
            }
            FtStack::merge(b, a, size, order, cmd)
        }
    }

    fn sort_push(a: &mut FtStack, b: &mut FtStack, size: usize, order: Ordering, cmd: &mut VecDeque<Cmd>) {
        if size < 3 {
            FtStack::sort_small(a, size, order.reverse(), cmd);
            for _ in 0..size {
                a.p(b, cmd);
            }
        } else {
            FtStack::sort_push(a, b, size - 2 * (size / 3), order, cmd);
            for _ in 0..(size - 2 * (size / 3)) {
                b.r(cmd);
            }
            FtStack::sort(a, b, size / 3, order, cmd);
            for _ in 0..(size / 3) {
                a.r(cmd);
            }
            FtStack::sort(a, b, size / 3, order.reverse(), cmd);
            FtStack::merge(a, b, size, order, cmd)
        }
    }
    
    fn merge(a: &mut FtStack, b: &mut FtStack, size: usize, order: Ordering, cmd: &mut VecDeque<Cmd>) {
        let mut a_top = size / 3;
        let mut a_bottom = size / 3;
        let mut b_bottom = size - 2 * (size / 3);
        for _ in 0..size {
            if a_top != 0 && a_bottom != 0 && b_bottom != 0 {
                if a.dq.front().unwrap().cmp(&a.dq.back().unwrap()) == order {
                    if a.dq.front().unwrap().cmp(&b.dq.back().unwrap()) == order {
                        a.p(b, cmd);
                        a_top -= 1;
                    } else {
                        b.rr(cmd);
                        b_bottom -= 1;
                    }       
                } else {
                    if a.dq.back().unwrap().cmp(&b.dq.back().unwrap()) == order {
                        a.rr(cmd);
                        a.p(b, cmd);
                        a_bottom -= 1;
                    } else {
                        b.rr(cmd);
                        b_bottom -= 1;
                    } 
                }
            } else if a_top != 0 && a_bottom != 0 {
                if a.dq.front().unwrap().cmp(&a.dq.back().unwrap()) == order {
                    a.p(b, cmd);
                    a_top -= 1;
                } else {
                    a.rr(cmd);
                    a.p(b, cmd);
                    a_bottom -= 1;   
                }
            } else if a_top != 0 && b_bottom != 0 {
                if a.dq.front().unwrap().cmp(&b.dq.back().unwrap()) == order {
                    a.p(b, cmd);
                    a_top -= 1;
                } else {
                    b.rr(cmd);
                    b_bottom -= 1;   
                }
            } else if a_bottom != 0 && b_bottom != 0 {
                if a.dq.back().unwrap().cmp(&b.dq.back().unwrap()) == order {
                    a.rr(cmd);
                    a.p(b, cmd);
                    a_bottom -= 1;
                } else {
                    b.rr(cmd);
                    b_bottom -= 1;   
                }
            } else if a_top != 0 {
                a.p(b, cmd);
                a_top -= 1;
            } else if a_bottom != 0 {
                a.rr(cmd);
                a.p(b, cmd);
                a_bottom -= 1;
            } else if b_bottom != 0 {
                b.rr(cmd);
                b_bottom -= 1;
            }
        }
    }

    fn sort_small(a: &mut FtStack, size: usize, order: Ordering, cmd: &mut VecDeque<Cmd>) {
        if size == 2 && a.dq.front().unwrap().cmp(&a.dq[1]) == order {
            a.s(cmd)
        }
    }

    fn s(&mut self, cmd: &mut VecDeque<Cmd>) {
        if self.dq.len() > 1 {
            self.dq.swap(0, 1);
            if self.name == 'a' {
                Self::sa(cmd)
            } else {
                Self::sb(cmd)
            }
        }
    }

    fn p(&mut self, b: &mut FtStack, cmd: &mut VecDeque<Cmd>) {
        if !self.dq.is_empty() {
            b.dq.push_front(self.dq.pop_front().unwrap());
            if self.name == 'a' {
                Self::pb(cmd)
            } else {
                Self::pa(cmd)
            }
        }
    }

    fn r(&mut self, cmd: &mut VecDeque<Cmd>) {
        if !self.dq.is_empty() {
            self.dq.rotate_left(1);
            if self.name == 'a' {
                Self::ra(cmd)
            } else {
                Self::rb(cmd)
            }
        }
    }

    fn rr(&mut self, cmd: &mut VecDeque<Cmd>) {
        if !self.dq.is_empty() {
            self.dq.rotate_right(1);
            if self.name == 'a' {
                Self::rra(cmd)
            } else {
                Self::rrb(cmd)
            }
        }
    }

    fn sa(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::SA) => {cmd.pop_back();},
            Some(Cmd::SB) => {
                cmd.pop_back();
                Self::ss(cmd);
            },
            Some(Cmd::SS) => {
                cmd.pop_back();
                Self::sb(cmd);
            },
            _ => cmd.push_back(Cmd::SA),
        }
    }

    fn sb(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::SA) => {
                cmd.pop_back();
                Self::ss(cmd);
            },
            Some(Cmd::SB) => {cmd.pop_back();},
            Some(Cmd::SS) => {
                cmd.pop_back();
                Self::sa(cmd);
            },
            _ => cmd.push_back(Cmd::SB),
        }
    }

    fn ss(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::SA) => {
                cmd.pop_back();
                Self::sb(cmd);
            },
            Some(Cmd::SB) => {
                cmd.pop_back();
                Self::sa(cmd);
            },
            Some(Cmd::SS) => {cmd.pop_back();},
            _ => cmd.push_back(Cmd::SS),
        }
    }

    fn pa(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::PB) => {cmd.pop_back();},
            _ => cmd.push_back(Cmd::PA),
        }
    }

    fn pb(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::PA) => {cmd.pop_back();},
            _ => cmd.push_back(Cmd::PB),
        }
    }

    fn ra(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::RRA) => {cmd.pop_back();},
            Some(Cmd::RB) => {
                cmd.pop_back();
                Self::_rr(cmd);
            },
            Some(Cmd::RRR) => {
                cmd.pop_back();
                Self::rrb(cmd);
            },
            _ => cmd.push_back(Cmd::RA),
        }
    }

    fn rb(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::RRB) => {cmd.pop_back();},
            Some(Cmd::RA) => {
                cmd.pop_back();
                Self::_rr(cmd);
            },
            Some(Cmd::RRR) => {
                cmd.pop_back();
                Self::rra(cmd);
            },
            _ => cmd.push_back(Cmd::RB),
        }
    }

    fn _rr(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::RRA) => {
                cmd.pop_back();
                Self::rb(cmd);
            },
            Some(Cmd::RRB) => {
                cmd.pop_back();
                Self::ra(cmd);
            },
            Some(Cmd::RRR) => {cmd.pop_back();},
            _ => cmd.push_back(Cmd::RR),
        }
    }

    fn rra(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::RA) => {cmd.pop_back();},
            Some(Cmd::RRB) => {
                cmd.pop_back();
                Self::rrr(cmd);
            },
            Some(Cmd::RR) => {
                cmd.pop_back();
                Self::rb(cmd);
            },
            _ => cmd.push_back(Cmd::RRA),
        }
    }

    fn rrb(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::RB) => {cmd.pop_back();},
            Some(Cmd::RRA) => {
                cmd.pop_back();
                Self::rrr(cmd);
            },
            Some(Cmd::RR) => {
                cmd.pop_back();
                Self::ra(cmd);
            },
            _ => cmd.push_back(Cmd::RRB),
        }
    }

    fn rrr(cmd: &mut VecDeque<Cmd>) {
        match cmd.back() {
            Some(Cmd::RA) => {
                cmd.pop_back();
                Self::rrb(cmd);
            },
            Some(Cmd::RB) => {
                cmd.pop_back();
                Self::rra(cmd);
            },
            Some(Cmd::RR) => {cmd.pop_back();},
            _ => cmd.push_back(Cmd::RRR),
        }
    }

    pub fn prt_cmd(cmd: Cmd) {
        match cmd {
            Cmd::SA => println!("sa"),
            Cmd::SB => println!("sb"),
            Cmd::SS => println!("ss"),
            Cmd::PA => println!("pa"),
            Cmd::PB => println!("pb"),
            Cmd::RA => println!("ra"),
            Cmd::RB => println!("rb"),
            Cmd::RR => println!("rr"),
            Cmd::RRA => println!("rra"),
            Cmd::RRB => println!("rrb"),
            Cmd::RRR => println!("rrr"),
        }
    }
}
