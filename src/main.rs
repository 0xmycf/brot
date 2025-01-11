use std::{
    char::from_u32,
    env,
    fmt::{Display, Write},
    fs,
    io::{stdin, Read},
    process::exit,
};

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
enum Instr {
    /// +
    Plus = 43,
    /// -
    Minus = 45,
    /// <
    SLeft = 60,
    /// >
    SRight = 62,
    /// [
    OBrack = 91,
    /// ]
    CBrack = 93,
    /// ,
    Comma = 44,
    /// .
    Dot = 46,
}

impl From<Instr> for char {
    fn from(value: Instr) -> Self {
        value as u8 as char
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

impl TryInto<Instr> for u8 {
    type Error = u8;

    fn try_into(self) -> Result<Instr, Self::Error> {
        match self {
            // +
            43 => Ok(Instr::Plus),
            // ,
            44 => Ok(Instr::Comma),
            // -
            45 => Ok(Instr::Minus),
            // -
            46 => Ok(Instr::Dot),
            // <
            60 => Ok(Instr::SLeft),
            // >
            62 => Ok(Instr::SRight),
            // [
            91 => Ok(Instr::OBrack),
            // ]
            93 => Ok(Instr::CBrack),
            a => Err(a),
        }
    }
}

struct Ctx {
    line: usize,
    col: usize,
}

impl Ctx {
    fn new() -> Self {
        Ctx { line: 0, col: 0 }
    }

    fn advance(&mut self, c: u8) {
        if b'\n' == c {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}

fn interpret(prog: Vec<u8>) {
    let mut cell: Vec<i32> = vec![0];
    let mut ptr: usize = 0;
    let mut ctx = Ctx::new();

    let mut read_buf: [u8; 1] = [0];
    let mut stdin = stdin();

    let mut idx = 0;
    while idx < prog.len() {
        let b = prog[idx];
        if let Ok(instr) = b.try_into() {
            match instr {
                Instr::Plus => cell[ptr] += 1,
                Instr::Minus => cell[ptr] -= 1,

                Instr::SLeft => match ptr.checked_sub(1) {
                    None => {
                        eprintln!("ptr went negative at {}:{}", ctx.line, ctx.col);
                        exit(1);
                    }

                    Some(x) => {
                        ptr = x;
                    }
                },
                Instr::SRight => {
                    ptr += 1;
                    if cell.len() - 1 <= ptr {
                        cell.push(0);
                    }
                }

                Instr::OBrack => {
                    if cell[ptr] == 0 {
                        while idx < prog.len() && prog[idx] != b']' {
                            idx += 1;
                        }
                        continue;
                    }
                }
                Instr::CBrack => {
                    if cell[ptr] != 0 {
                        while idx < prog.len() && prog[idx] != b'[' {
                            idx -= 1;
                        }
                        continue;
                    }
                }

                Instr::Comma => {
                    if let Err(err) = stdin.read_exact(&mut read_buf) {
                        eprintln!(
                            "Error ({}) while reading a single from stdin at {}:{}",
                            err, ctx.line, ctx.col
                        );
                        exit(1);
                    };
                    cell[ptr] = read_buf[0] as i32;
                }
                Instr::Dot => {
                    let value = if cell[ptr] < 32 {
                        cell[ptr].to_string()
                    } else {
                        from_u32(cell[ptr] as u32)
                            .map(|c| c.to_string())
                            .unwrap_or(cell[ptr].to_string())
                    };
                    print!("{value}");
                }
            }
        } else { /* Do nothing */
        }
        ctx.advance(b);
        idx += 1;
    }
}

/**
    Exit 0 -- all fine
    Exit 1 -- Some error occured (see stderr)
*/
fn main() {
    let file_name = env::args().nth(1).unwrap_or("./data/file.test".to_string());
    let file = fs::read(file_name).expect("Error while reading the file");

    interpret(file);
    exit(0);
}
