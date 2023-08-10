use super::*;

impl<R: Read> Lex<R> {
  pub(super) fn check_ahead(
    &mut self,
    ahead_cases: Vec<char>,
    candidates: Vec<Token>,
    failed: Token,
  ) -> Token {
    let ch = self.peek_char();
    for (ahead, candidate) in ahead_cases.into_iter().zip(candidates) {
      if ch == ahead {
        self.next_char();
        return candidate;
      }
    }
    failed
  }
}

impl<R: Read> Lex<R> {
  pub(super) fn lex_name(&mut self, first: char) -> Token {
    let mut name = String::new();
    name.push(first);
    loop {
      let c = self.peek_char();
      if c.is_alphanumeric() || c == '_' {
        self.next_char();
        name.push(c);
      } else {
        break;
      }
    }
    // check => if it's keyword
    // TODO: add support of `flexible reserved identifier (such as async/await)`
    match &name as &str {
      "and" => Token::And,
      "break" => Token::Break,
      "do" => Token::Do,
      "else" => Token::Else,
      "elseif" => Token::Elseif,
      "end" => Token::End,
      "false" => Token::False,
      "for" => Token::For,
      "function" => Token::Function,
      "goto" => Token::Goto,
      "if" => Token::If,
      "in" => Token::In,
      "local" => Token::Local,
      "nil" => Token::Nil,
      "not" => Token::Not,
      "or" => Token::Or,
      "repeat" => Token::Repeat,
      "return" => Token::Return,
      "then" => Token::Then,
      "true" => Token::True,
      "until" => Token::Until,
      "while" => Token::While,
      _ => Token::Name(name),
    }
  }

  pub(super) fn lex_string(&mut self, ending: char) -> Token {
    let mut string = String::new();
    loop {
      match self.read_char() {
        '\n' | '\0' => panic!("unfinished string"),
        '\\' => unimplemented!("escape"),
        c if c == ending => break,
        c => string.push(c),
      }
    }
    Token::String(string)
  }

  pub(super) fn lex_comment(&mut self) {
    /* `--` has been read */
    match self.read_char() {
      '[' => unimplemented!("multiline/long comment"),
      _ => loop {
        /* single line comment (end at `\n` or `\0`) */
        let c = self.read_char();
        if c == '\n' || c == '\0' {
          break;
        }
      },
    }
  }

  fn lex_number_in_radix(&mut self, radix: u32) -> Token {
    let mut scanned = 0;
    loop {
      let c = self.peek_char();
      let curr = char::to_digit(c, radix);
      match curr {
        Some(curr) => {
          self.next_char();
          scanned = scanned * radix as i64 + curr as i64;
        }
        None => match c {
          '.' => {
            self.next_char();
            return self.lex_number_fraction_in_radix(scanned, radix);
          }
          _ => {
            break;
          }
        },
      }
    }
    // check following
    let following = self.peek_char();
    if following.is_alphabetic() || following == '.' {
      panic!("malformed number");
    }
    // Ok
    Token::Integer(scanned)
  }

  pub(super) fn lex_number(&mut self, first: char) -> Token {
    if first == '0' {
      let second = self.peek_char();
      match second {
        'x' | 'X' => {
          self.next_char();
          return self.lex_number_in_radix(16);
        }
        'b' | 'B' => {
          self.next_char();
          return self.lex_number_in_radix(2);
        }
        'o' | 'O' => {
          self.next_char();
          return self.lex_number_in_radix(8);
        }
        _ => {}
      }
    }
    let mut scanned = char::to_digit(first, 10).unwrap() as i64;
    loop {
      let c = self.peek_char();
      let curr = char::to_digit(c, 10);
      match curr {
        Some(curr) => {
          self.next_char();
          scanned = scanned * 10 + curr as i64
        }
        None => match c {
          '.' => {
            self.next_char();
            return self.lex_number_fraction(scanned);
          }
          'e' | 'E' => {
            self.next_char();
            return self.lex_number_exponent(scanned as f64);
          }
          _ => {
            break;
          }
        },
      }
    }
    // check following
    let following = self.peek_char();
    if following.is_alphabetic() || following == '.' {
      panic!("malformed number");
    }
    // Ok
    Token::Integer(scanned)
  }

  pub(super) fn lex_number_exponent(&mut self, original: f64) -> Token {
    let first = self.read_char();
    let mut scanned = 0;
    let mut sign = 1;
    match first {
      '+' => {}
      '-' => sign = -1,
      _ => scanned = char::to_digit(first, 10).unwrap() as i64,
    }
    loop {
      let c = self.peek_char();
      let curr = char::to_digit(c, 10);
      if let Some(curr) = curr {
        self.next_char();
        scanned = scanned * 10 + curr as i64
      } else {
        break;
      }
    }
    Token::Float(original * 10f64.powi(scanned as i32 * sign))
  }

  fn lex_number_fraction_in_radix(&mut self, original: i64, radix: u32) -> Token {
    let mut scanned = 0;
    let mut bits = 1.0;
    loop {
      let c = self.peek_char();
      let curr = char::to_digit(c, radix);
      if let Some(curr) = curr {
        self.next_char();
        scanned = scanned * radix as i64 + curr as i64;
        bits *= radix as f64;
      } else {
        break;
      }
    }
    Token::Float(original as f64 + scanned as f64 / bits)
  }

  pub(super) fn lex_number_fraction(&mut self, original: i64) -> Token {
    let mut scanned = 0;
    let mut bits = 1.0;
    loop {
      let c = self.peek_char();
      match char::to_digit(c, 10) {
        Some(curr) => {
          self.next_char();
          scanned = scanned * 10 + curr as i64;
          bits *= 10.0;
        }
        None => match c {
          'e' | 'E' => {
            self.next_char();
            return self.lex_number_exponent(original as f64 + scanned as f64 / bits);
          }
          _ => {
            break;
          }
        },
      }
    }
    Token::Float(original as f64 + scanned as f64 / bits)
  }
}
