// We allow non_camel case here as this makes it easier to distingush what
// we're working with
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum Token {
    PLUS = 43,
    MINUS = 45,

    GREATER = 60,
    SMALLER = 62,

    DOT = 46,
    COMMA = 44,

    LEFT_BRACKET = 91,
    RIGHT_BRACKET = 93,
}

impl TryFrom<u8> for Token {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use self::Token::*;
        match value {
            // +
            43 => Ok(PLUS),
            // ,
            44 => Ok(COMMA),
            // -
            45 => Ok(MINUS),
            // -
            46 => Ok(DOT),
            // <
            60 => Ok(SMALLER),
            // >
            62 => Ok(GREATER),
            // [
            91 => Ok(LEFT_BRACKET),
            // ]
            93 => Ok(RIGHT_BRACKET),
            _ => Err(()),
        }
    }
}

impl Token {
    fn is_loop_beginning(&self) -> bool {
        // why a macro over some basic langauge match syntax, clippy?
        matches!(self, Self::LEFT_BRACKET)
    }
}

/// The parsed "AST"
#[derive(Debug, PartialEq)]
pub enum Syntax {
    /// +
    Incr(usize),
    /// -
    Decr(usize),

    /// <
    ShiftL(usize),
    /// >
    ShiftR(usize),

    /// .
    Write(usize),
    /// ,
    Read(usize),

    // []
    Loop(Vec<Syntax>),
}

impl Syntax {
    fn from_token(token: Token, amount: usize) -> Syntax {
        use Syntax::*;
        use Token::*;

        match token {
            PLUS => Incr(amount),
            MINUS => Decr(amount),
            // >
            GREATER => ShiftR(amount),
            // <
            SMALLER => ShiftL(amount),
            DOT => Write(amount),
            COMMA => Read(amount),
            LEFT_BRACKET | RIGHT_BRACKET => unreachable!(),
        }
    }
}

pub mod lexer {
    use super::Token::{self};

    pub fn lex(input: &str) -> impl Iterator<Item = Token> {
        let mut out: Vec<Token> = vec![];
        for c in input.bytes() {
            if let Ok(token) = c.try_into() {
                out.push(token);
            }
        }
        out.into_iter()
    }

    #[cfg(test)]
    mod tests {
        use super::Token::*;
        use super::*;

        #[test]
        fn lex_pure() {
            let input = "+-<>.,[]";
            let expected = vec![
                PLUS,
                MINUS,
                SMALLER,
                GREATER,
                DOT,
                COMMA,
                LEFT_BRACKET,
                RIGHT_BRACKET,
            ];
            assert_eq!(expected, lex(input).collect::<Vec<Token>>());
        }

        #[test]
        fn with_trash() {
            let input = "+ -         < something else > 1235 .


                ,[ I 
                is this how multiline strings 

    work in the
                                                        rust programming language  

                                        Γ ⊢ φ : τ₁ ⊣ e : ℳ


                ]";
            let expected = vec![
                PLUS,
                MINUS,
                SMALLER,
                GREATER,
                DOT,
                COMMA,
                LEFT_BRACKET,
                RIGHT_BRACKET,
            ];
            assert_eq!(expected, lex(input).collect::<Vec<Token>>());
        }

        #[test]
        fn token_collatz() {
            let col = ">,[
                            [
                                ----------[
                                    >>>[>>>>]+[[-]+<[->>>>++>>>>+[>>>>]++[->+<<<<<]]<<<]
                                    ++++++[>------<-]>--[>>[->>>>]+>+[<<<<]>-],<
                                ]>
                            ]>>>++>+>>[
                                <<[>>>>[-]+++++++++<[>-<-]+++++++++>[-[<->-]+[<<<<]]<[>+<-]>]
                                >[>[>>>>]+[[-]<[+[->>>>]>+<]>[<+>[<<<<]]+<<<<]>>>[->>>>]+>+[<<<<]]
                                >[[>+>>[<<<<+>>>>-]>]<<<<[-]>[-<<<<]]>>>>>>>
                            ]>>+[[-]++++++>>>>]<<<<[[<++++++++>-]<.[-]<[-]<[-]<]<,
                        ]";
            let result = lex(col);
            let len = col
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
                .len();
            let tmp = result.collect::<Vec<Token>>();
            assert_eq!(len, tmp.len())
        }
    }
}

pub mod parser {
    use super::{Syntax, Token};

    pub fn parse(iter: impl Iterator<Item = Token>) -> Result<Vec<Syntax>, String> {
        let mut out: Vec<Syntax> = vec![];
        let mut peek = iter.peekable();
        while let Some(token) = peek.next() {
            if token.is_loop_beginning() {
                let mut buf = vec![];
                let mut depth = 1;
                for tkn2 in peek.by_ref() {
                    // thanks clippy
                    if tkn2 == Token::LEFT_BRACKET {
                        depth += 1;
                    } else if tkn2 == Token::RIGHT_BRACKET {
                        depth -= 1;
                    }
                    if depth == 0 {
                        break;
                    }
                    buf.push(tkn2);
                }
                if depth != 0 {
                    return Err("Mismatched brackets in loop".to_owned());
                }
                let inner = parse(buf.into_iter())?;
                out.push(Syntax::Loop(inner));
            } else if token == Token::RIGHT_BRACKET {
                return Err("Found orphaned ]".to_owned());
            } else {
                let mut amount = 1;
                while let Some(tkn2) = peek.peek() {
                    if *tkn2 != token {
                        break;
                    }
                    peek.next();
                    amount += 1;
                }
                out.push(Syntax::from_token(token, amount))
            }
        }
        Ok(out)
    }

    #[cfg(test)]
    mod tests {
        //{{{
        use super::Token::*;
        use super::*;

        type TestResult = Result<(), Box<dyn std::error::Error>>;

        #[test]
        fn parse_5_plus() -> TestResult {
            let in_ = vec![PLUS, PLUS, PLUS, PLUS, PLUS];
            let result = parse(in_.into_iter())?;
            assert_eq!(vec![Syntax::Incr(5)], result);
            Ok(())
        }

        #[test]
        fn parse_5_minus() -> TestResult {
            let in_ = vec![MINUS, MINUS, MINUS, MINUS, MINUS];
            let result = parse(in_.into_iter())?;
            assert_eq!(vec![Syntax::Decr(5)], result);
            Ok(())
        }

        #[test]
        fn parse_5_greater() -> TestResult {
            // >
            let in_ = vec![GREATER, GREATER, GREATER, GREATER, GREATER];
            let result = parse(in_.into_iter())?;
            assert_eq!(vec![Syntax::ShiftR(5)], result);
            Ok(())
        }

        #[test]
        fn parse_5_smaller() -> TestResult {
            // <
            let in_ = vec![SMALLER, SMALLER, SMALLER, SMALLER, SMALLER];
            let result = parse(in_.into_iter())?;
            assert_eq!(vec![Syntax::ShiftL(5)], result);
            Ok(())
        }

        #[test]
        fn parse_empty_loop() -> TestResult {
            let in_ = vec![LEFT_BRACKET, RIGHT_BRACKET];
            let result = parse(in_.into_iter())?;
            assert_eq!(vec![Syntax::Loop(vec![])], result);
            Ok(())
        }

        #[test]
        fn parse_before_and_after_loop() -> TestResult {
            let in_ = vec![
                PLUS,
                MINUS,
                MINUS,
                LEFT_BRACKET,
                RIGHT_BRACKET,
                GREATER,
                SMALLER,
            ];
            let result = parse(in_.into_iter())?;
            use Syntax::*;
            assert_eq!(
                vec![Incr(1), Decr(2), Syntax::Loop(vec![]), ShiftR(1), ShiftL(1)],
                result
            );
            Ok(())
        }

        #[test]
        fn parse_with_body_flat() -> TestResult {
            let in_ = vec![LEFT_BRACKET, PLUS, PLUS, MINUS, RIGHT_BRACKET];
            let result = parse(in_.into_iter())?;
            use Syntax::*;
            assert_eq!(vec![Syntax::Loop(vec![Incr(2), Decr(1)])], result);
            Ok(())
        }

        #[test]
        fn parse_with_body_nested() -> TestResult {
            let in_ = vec![
                LEFT_BRACKET,
                PLUS,
                LEFT_BRACKET,
                PLUS,
                RIGHT_BRACKET,
                MINUS,
                RIGHT_BRACKET,
            ];
            let result = parse(in_.into_iter())?;
            use Syntax::*;
            assert_eq!(
                vec![Syntax::Loop(vec![Incr(1), Loop(vec![Incr(1)]), Decr(1)])],
                result
            );
            Ok(())
        }

        #[test]
        fn parse_incorrect_brackets() -> TestResult {
            let in_ = vec![LEFT_BRACKET, PLUS, PLUS, PLUS];
            let err = parse(in_.into_iter());
            assert_eq!(true, err.is_err());
            Ok(())
        }

        #[test]
        fn parse_incorrect_brackets_flipped() -> TestResult {
            let in_ = vec![RIGHT_BRACKET, LEFT_BRACKET];
            let err = parse(in_.into_iter());
            assert_eq!(true, err.is_err());
            Ok(())
        }
    } //}}}
}

pub mod interpreter {
    use std::{
        ascii,
        io::{stdin, stdout, Read, Write},
        process::exit,
    };

    use termion::{cursor::DetectCursorPos, raw::IntoRawMode};

    use super::*;

    #[derive(Debug)]
    pub struct State {
        pub ptr: usize,
        pub tape: Vec<i32>,
    }

    enum Shift {
        Left,
        Right,
    }

    impl State {
        fn new() -> Self {
            State {
                ptr: 0,
                tape: vec![0],
            }
        }

        fn shift(&mut self, shift: Shift, amount: usize) {
            match shift {
                Shift::Left => {
                    self.ptr = self.ptr.checked_sub(amount)
                        .expect("Cannot go lower than 0 for tape address indexing (more < than > in your program)");
                }
                Shift::Right => {
                    self.ptr += amount;

                    while self.tape.len() <= self.ptr {
                        self.tape.push(0);
                    }
                }
            }
        }

        pub fn current_value(&self) -> i32 {
            self.tape[self.ptr]
        }

        fn incr(&mut self, amount: usize) {
            self.tape[self.ptr] += amount as i32;
        }

        fn decr(&mut self, amount: usize) {
            self.tape[self.ptr] -= amount as i32;
        }

        fn set_value(&mut self, buf_value: u8) {
            self.tape[self.ptr] = buf_value as i32;
        }

        fn is_zero(&self) -> bool {
            self.current_value() == 0
        }
    }

    fn format_for_ttv(value: i32) -> String {
        match u8::try_from(value) {
            Ok(u) => format!("{}", ascii::escape_default(u)),
            Err(_) => format!("{}", value),
        }
    }

    fn interpret(stmt: &Syntax, state: &mut State) {
        match stmt {
            Syntax::Incr(i) => state.incr(*i),
            Syntax::Decr(i) => state.decr(*i),
            Syntax::ShiftL(n) => state.shift(Shift::Left, *n),
            Syntax::ShiftR(n) => state.shift(Shift::Right, *n),
            Syntax::Write(n) => {
                let value = state.current_value();
                let s = format_for_ttv(value);
                for _ in 1..=*n {
                    print!("{s}");
                }
            }
            /*
                Prepare for terminal magic...
            */
            Syntax::Read(n) => {
                // it makes little sense to continue here does it?
                let mut stdout = stdout()
                    .into_raw_mode() // we enter raw mode to easily get the cursor pos
                    .expect("Cannot shift terminal into raw mode");
                for _ in 1..=*n {
                    let mut buf = [0_u8];
                    let (x, y) = stdout
                        .cursor_pos()
                        .expect("Cannot get current cursor position");
                    let _ = stdout.flush();
                    print!("_");
                    let _ = stdout.flush();
                    if let Err(err) = stdin().read_exact(&mut buf) {
                        panic!("{}", err);
                    }

                    // ctrl c should always exit the program
                    if b'\x03' == buf[0] {
                        exit(0);
                    }

                    let escaped_value = ascii::escape_default(buf[0]);
                    print!("{}{}", termion::cursor::Goto(x, y), escaped_value);
                    let _ = stdout.flush();
                    state.set_value(buf[0]);
                }
            }
            Syntax::Loop(v) => {
                while !state.is_zero() {
                    for ele in v {
                        interpret(ele, state);
                    }
                }
            }
        }
    }

    // returns the final result
    pub fn run(program: Vec<Syntax>) -> State {
        let mut s = State::new();
        for x in program.iter() {
            interpret(x, &mut s);
        }
        s
    }
}
