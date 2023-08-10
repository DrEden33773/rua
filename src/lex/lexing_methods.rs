use super::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS_MAP: Lazy<HashMap<&str, Token>> = Lazy::new(|| {
  vec![
    ("and", Token::And),
    ("break", Token::Break),
    ("do", Token::Do),
    ("else", Token::Else),
    ("elseif", Token::Elseif),
    ("end", Token::End),
    ("false", Token::False),
    ("for", Token::For),
    ("function", Token::Function),
    ("goto", Token::Goto),
    ("if", Token::If),
    ("in", Token::In),
    ("local", Token::Local),
    ("nil", Token::Nil),
    ("not", Token::Not),
    ("or", Token::Or),
    ("repeat", Token::Repeat),
    ("return", Token::Return),
    ("then", Token::Then),
    ("true", Token::True),
    ("until", Token::Until),
    ("while", Token::While),
  ]
  .into_iter()
  .map(|(k, v)| (k, v))
  .collect::<HashMap<_, _>>()
});

impl<R: Read> Lex<R> {
  pub(super) fn check_ahead(
    &mut self,
    ahead_cases: Vec<u8>,
    candidates: Vec<Token>,
    failed: Token,
  ) -> Token {
    let ch = self.peek_byte();
    for (ahead, candidate) in ahead_cases.into_iter().zip(candidates) {
      if ch == ahead {
        self.next_byte();
        return candidate;
      }
    }
    failed
  }
}

impl<R: Read> Lex<R> {
  pub(super) fn lex_name(&mut self, first: u8) -> Token {
    let mut name = String::new();
    name.push(first as char);
    loop {
      let c = self.peek_byte() as char;
      if c.is_alphanumeric() || c == '_' {
        self.next_byte();
        name.push(c);
      } else {
        break;
      }
    }
    // check => if it's keyword
    // TODO: add support of `flexible reserved identifier (such as async/await)`
    KEYWORDS_MAP
      .get(&name[..])
      .cloned()
      .unwrap_or(Token::Name(name))
  }

  pub(super) fn lex_string(&mut self, ending: u8) -> Token {
    let mut string = vec![];
    loop {
      match self.next_byte().expect("unfinished string") {
        b'\n' => string.push(self.lex_string_escape()),
        b'\\' => unimplemented!("string escape"),
        c if c == ending => break,
        c => string.push(c),
      }
    }
    Token::String(string)
  }

  fn lex_string_escape(&mut self) -> u8 {
    match self.next_byte().expect("string escape") {
      b'a' => 0x07,
      b'b' => 0x08,
      b'f' => 0x0c,
      b'v' => 0x0b,
      b'n' => b'\n',
      b'r' => b'\r',
      b't' => b'\t',
      b'\\' => b'\\',
      b'"' => b'"',
      b'\'' => b'\'',
      b'x' => {
        // format: \xXX
        let lhs = char::to_digit(self.next_byte().unwrap() as char, 16).unwrap();
        let rhs = char::to_digit(self.next_byte().unwrap() as char, 16).unwrap();
        (lhs * 16 + rhs) as u8
      }
      ch @ b'0'..=b'9' => {
        // format: \d[d[d]]
        let mut scanned = char::to_digit(ch as char, 10).unwrap();
        if let Some(curr) = char::to_digit(self.peek_byte() as char, 10) {
          self.next_byte();
          scanned = scanned * 10 + curr;
          if let Some(d) = char::to_digit(self.peek_byte() as char, 10) {
            self.next_byte();
            scanned = scanned * 10 + d;
          }
        }
        u8::try_from(scanned).expect("decimal escape too large")
      }
      _ => panic!("invalid string escape"),
    }
  }

  pub(super) fn lex_comment(&mut self) {
    /* `--` has been read */
    match self.next_byte() {
      Some(b'[') => unimplemented!("multiline/long comment"),
      None => (),
      Some(_) => loop {
        /* single line comment (end at `\n` or `\0`) */
        while let Some(c) = self.next_byte() {
          if c == b'\n' {
            return;
          }
        }
      },
    }
  }

  fn lex_number_in_radix(&mut self, radix: u32) -> Token {
    let mut scanned = 0;
    loop {
      let c = self.peek_byte();
      let curr = char::to_digit(c as char, radix);
      match curr {
        Some(curr) => {
          self.next_byte();
          scanned = scanned * radix as i64 + curr as i64;
        }
        None => match c {
          b'.' => {
            self.next_byte();
            return self.lex_number_fraction_in_radix(scanned, radix);
          }
          _ => {
            break;
          }
        },
      }
    }
    // check following
    let following = self.peek_byte() as char;
    if following.is_alphabetic() || following == '.' {
      panic!("malformed number");
    }
    // Ok
    Token::Integer(scanned)
  }

  pub(super) fn lex_number(&mut self, first: u8) -> Token {
    if first == b'0' {
      let second = self.peek_byte();
      match second {
        b'x' | b'X' => {
          self.next_byte();
          return self.lex_number_in_radix(16);
        }
        b'b' | b'B' => {
          self.next_byte();
          return self.lex_number_in_radix(2);
        }
        b'o' | b'O' => {
          self.next_byte();
          return self.lex_number_in_radix(8);
        }
        _ => {}
      }
    }
    let mut scanned = (first - b'0') as i64;
    loop {
      let c = self.peek_byte();
      let curr = char::to_digit(c as char, 10);
      match curr {
        Some(curr) => {
          self.next_byte();
          scanned = scanned * 10 + curr as i64
        }
        None => match c {
          b'.' => {
            self.next_byte();
            return self.lex_number_fraction(scanned);
          }
          b'e' | b'E' => {
            self.next_byte();
            return self.lex_number_exponent(scanned as f64);
          }
          _ => {
            break;
          }
        },
      }
    }
    // check following
    let following = self.peek_byte() as char;
    if following.is_alphabetic() || following == '.' {
      panic!("malformed number");
    }
    // Ok
    Token::Integer(scanned)
  }

  pub(super) fn lex_number_exponent(&mut self, original: f64) -> Token {
    let first = self.read_byte();
    let mut scanned = 0;
    let mut sign = 1;
    match first {
      b'+' => {}
      b'-' => sign = -1,
      _ => scanned = char::to_digit(first as char, 10).unwrap() as i64,
    }
    loop {
      let c = self.peek_byte();
      let curr = char::to_digit(c as char, 10);
      if let Some(curr) = curr {
        self.next_byte();
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
      let c = self.peek_byte();
      let curr = char::to_digit(c as char, radix);
      if let Some(curr) = curr {
        self.next_byte();
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
      let c = self.peek_byte();
      match char::to_digit(c as char, 10) {
        Some(curr) => {
          self.next_byte();
          scanned = scanned * 10 + curr as i64;
          bits *= 10.0;
        }
        None => match c {
          b'e' | b'E' => {
            self.next_byte();
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
