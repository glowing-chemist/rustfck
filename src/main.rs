#![feature(io)]
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

struct Programstack {
    stack: Vec<u32>,
    p: usize
}

impl Programstack {
    fn push(&mut self,val: u32) {
        while self.p >= self.stack.len() {
            self.stack.push(0);
        }
        self.stack[self.p] = val;
    }

    fn advance(&mut self) {
        self.p += 1;
        while self.p >= self.stack.len() {
            self.stack.push(0);
        }
    }

    fn read(&self) -> u32 {
        self.stack[self.p].clone()
    }

    fn add(&mut self) {
        while self.p >= self.stack.len() {
            self.stack.push(0);
        }
        self.stack[self.p] += 1;
    }

    fn sub(&mut self) {
         while self.p >= self.stack.len() {
            self.stack.push(0);
        }
        self.stack[self.p] -= 1;
    }
}

fn read_file(fil: String) -> Vec<char> {
    let f = File::open(fil).unwrap();
    let mut ve = Vec::new();
    for c in f.chars() {
        ve.push(c.unwrap());
    }
    ve
}

fn interpr(program: &[char],pstack: &mut Programstack) {
    let mut loopflag = 0;
    let mut loopvar: Vec<char> = Vec::new();
    for c in program {
        if loopflag > 0 {
            if *c == ']' {
                loopflag -= 1;
                if loopflag == 0{
                    while pstack.read() != 0 {
                        interpr(loopvar.as_slice(),pstack);
                    }
                loopvar = Vec::new();
                continue;
                }
            }
            loopvar.push(*c);
        }
        match *c {
            '>'  => pstack.advance(),
            '<'  => pstack.p -= 1,
            '+'  => pstack.add(),
            '-'  => pstack.sub(),
            '.'  => println!("{}",pstack.read()),
            ','  => {
                    let mut si = String::new();
                    io::stdin().read_line(&mut si);
                    pstack.push(si.trim().parse::<u32>().unwrap());
                }
            '['  => loopflag += 1,
            '@'  => print!("{}\n", pstack.p),
             _   => continue
        };
    }
}


fn main() {
    let x = read_file(env::args().nth(1).unwrap());
    let prog = &mut Programstack{stack: Vec::new(),p: 0};
    interpr(x.as_slice(),prog);
}
