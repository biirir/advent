// https://adventofcode.com/2015/day/23

use simple_error::{bail, SimpleError};
use std::io::BufRead;

struct Reg(usize);

enum Op {
    Half(Reg),
    Triple(Reg),
    Inc(Reg),
    Jmp(i32),
    JmpOne(Reg, i32),
    JmpEven(Reg, i32),
}

struct Machine {
    ops: Vec<Op>,
    regs: [i32; 2],
}

impl Machine {
    fn new() -> Machine {
        Machine {
            ops: Vec::new(),
            regs: [0; 2],
        }
    }

    fn parse_add(&mut self, line: &str) -> Result<(), SimpleError> {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let reg = || -> Result<Reg, SimpleError> {
            match words[1] {
                "a" | "a," => Ok(Reg(0)),
                "b" | "b," => Ok(Reg(1)),
                _ => bail!("Unkown register {:?}", words[1]),
            }
        };
        let offset = |i: usize| words[i].parse().map_err(SimpleError::from);

        self.ops.push(match words[0] {
            "inc" => Op::Inc(reg()?),
            "hlf" => Op::Half(reg()?),
            "tpl" => Op::Triple(reg()?),
            "jmp" => Op::Jmp(offset(1)?),
            "jio" => Op::JmpOne(reg()?, offset(2)?),
            "jie" => Op::JmpEven(reg()?, offset(2)?),
            _ => bail!("Unkown instruction {:?}", words[0]),
        });
        Ok(())
    }

    fn execute(&mut self) {
        let lim = self.ops.len() as i32;
        let mut i = 0i32;

        while i >= 0 && i < lim {
            match self.ops[i as usize] {
                Op::Inc(Reg(r)) => self.regs[r] += 1,
                Op::Half(Reg(r)) => self.regs[r] /= 2,
                Op::Triple(Reg(r)) => self.regs[r] *= 3,
                Op::Jmp(offset) => {
                    i += offset;
                    continue;
                }
                Op::JmpOne(Reg(r), offset) => {
                    if self.regs[r] == 1 {
                        i += offset;
                        continue;
                    }
                }
                Op::JmpEven(Reg(r), offset) => {
                    if self.regs[r] % 2 == 0 {
                        i += offset;
                        continue;
                    }
                }
            }
            i += 1;
        }
    }
}

pub fn day23(filename: Option<&str>) -> (String, String) {
    let mut machine = Machine::new();

    for l in crate::bufread(filename.unwrap_or("input/23")).lines() {
        machine.parse_add(&l.unwrap()).unwrap();
    }

    // Part 1.
    machine.execute();
    let b0 = machine.regs[1];

    // Reset registers for part 2.
    machine.regs[0] = 1;
    machine.regs[1] = 0;

    machine.execute();
    let b1 = machine.regs[1];

    (b0.to_string(), b1.to_string())
}
