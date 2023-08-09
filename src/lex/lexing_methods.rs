use super::*;

impl Lex {
  pub(super) fn check_ahead(
    &mut self,
    ahead_cases: Vec<char>,
    candidates: Vec<Token>,
    failed: Token,
  ) -> Token {
    let ch = self.read_char();
    for (ahead, candidate) in ahead_cases.into_iter().zip(candidates.into_iter()) {
      if ch == ahead {
        return candidate;
      }
    }
    self.move_back();
    failed
  }
}

impl Lex {
  pub(super) fn lex_name(&mut self, first: char) -> Token {
    let mut name = String::new();
    name.push(first);
    loop {
      match self.read_char() {
        '\0' => break,
        '_' => name.push('_'),
        c if c.is_alphanumeric() => name.push(c),
        _ => {
          self.move_back();
          break;
        }
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

  pub(super) fn lex_hex(&mut self) -> Token {
    unimplemented!("lex hex")
  }

  pub(super) fn lex_number(&mut self, first: char) -> Token {
    /* HEX */
    if first == '0' {
      let second = self.read_char();
      if second == 'x' || second == 'X' {
        todo!()
      }
      self.move_back();
    }
    /* DEC */
    let mut scanned = char::to_digit(first, 10).unwrap() as i64;
    loop {
      let c = self.read_char();
      let curr = char::to_digit(c, 10);
      match curr {
        Some(curr) => scanned = scanned * 10 + curr as i64,
        None => match c {
          '.' => return self.lex_number_fraction(scanned),
          'e' | 'E' => return self.lex_number_exponent(scanned as f64),
          _ => {
            self.move_back();
            break;
          }
        },
      }
    }
    // check following
    let following = self.read_char();
    if following.is_alphabetic() || following == '.' {
      panic!("incorrect formatted number");
    }
    // Ok
    Token::Integer(scanned)
  }

  pub(super) fn lex_number_exponent(&mut self, original: f64) -> Token {
    unimplemented!("lex number in scientific notation")
  }

  pub(super) fn lex_number_fraction(&mut self, original: i64) -> Token {
    let mut scanned = 0;
    let mut bits = 1.0;
    loop {
      let c = self.read_char();
      if let Some(curr) = char::to_digit(c, 10) {
        scanned = scanned * 10 + curr as i64;
        bits *= 10.0;
      } else {
        self.move_back();
        break;
      }
    }
    Token::Float(original as f64 + scanned as f64 / bits)
  }
}
