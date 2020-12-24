// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A lexer for tokenizing input strings.

use super::{
  syn::{Token, Tokens},
  *,
};

/// A structure containing lexer state.
struct Lexer<'src> {
  errors: ErrorList,
  input: &'src str,
  open_delims: Vec<syn::Delimiter>,
  pos: Pos,
  tokens: Vec<Token>,
}

/// A value with associated span.
#[derive(Clone, Copy, Deref)]
struct Spanned<T> {
  span: Span,
  #[deref]
  value: T,
}

/// Parses tokens from a IDN input string.
pub fn lex(input: &str) -> Result<Tokens, ErrorList> {
  // Initialize the lexer.

  let mut lexer =
    Lexer { input, open_delims: default(), errors: default(), pos: default(), tokens: default() };

  // Run the lexer on the entire input.

  if let Err(err) = lexer.run() {
    lexer.errors.add(err);
  }

  // Return the results.

  match lexer.errors.len() {
    0 => Ok(Tokens::new(default()..lexer.pos, lexer.tokens)),
    _ => Err(lexer.errors),
  }
}

impl<'src> Lexer<'src> {
  /// Reads a token from the input.
  fn run(&mut self) -> Result<(), Error> {
    while let Some(c) = self.peek_char() {
      match c {
        '(' | '[' | '{' | ')' | ']' | '}' => {
          self.read_delimiter()?;
        }

        '"' | '\'' => {
          self.read_string()?;
        }

        '/' if self.peek_str_exact("//") => {
          self.read_comment()?;
        }

        '/' if self.peek_str_exact("/*") => {
          self.read_comment_multiline()?;
        }

        '.' if self.peek_number_decimal() => {
          self.read_number()?;
        }

        other if other.is_ascii_digit() => {
          self.read_number()?;
        }

        other if syn::Word::is_start_char(other) => {
          self.read_word_token()?;
        }

        other if other.is_whitespace() => {
          self.read_char()?;
        }

        _ => {
          self.read_symbol_token()?;
        }
      }
    }

    // Add an error for each unmatched delimiter.

    for d in self.open_delims.drain(..) {
      self.errors.add(err!(d.span(), "Unmatched `{}`.", d.as_char()));
    }

    Ok(())
  }

  /// Adds a token to the output.
  fn add_token(&mut self, token: impl Into<Token>) {
    self.tokens.push(token.into());
  }

  /// Returns `true` if no more input remains.
  fn is_eof(&self) -> bool {
    self.remaining().is_empty()
  }

  /// Returns the next character in the input string.
  fn peek_char(&mut self) -> Option<char> {
    self.remaining().chars().next()
  }

  /// Returns `true` if the input starts with the given character.
  fn peek_char_exact(&mut self, c: char) -> bool {
    self.remaining().chars().next() == Some(c)
  }

  /// Returns `true` if the remaining input starts with a `.` and at least one
  /// digit.
  fn peek_number_decimal(&mut self) -> bool {
    let mut lookahead = self.remaining().chars();

    lookahead.next() == Some('.') && matches!(lookahead.next(), Some(c) if c.is_ascii_digit())
  }

  /// Returns `true` if the remaining input starts with the given string.
  fn peek_str_exact(&mut self, string: &str) -> bool {
    self.remaining().starts_with(string)
  }

  /// Reads the next character from the input.
  fn read_char(&mut self) -> Result<Spanned<char>, Error> {
    let c = match self.peek_char() {
      Some(c) => c,
      None => abort!(self.pos, "Unexpected end of input."),
    };

    let start_pos = self.pos;

    self.pos.advance(c);

    Ok(Spanned::new(start_pos..self.pos, c))
  }

  /// Reads a specific character from the input.
  fn read_char_exact(&mut self, expected: char) -> Result<Spanned<char>> {
    if self.is_eof() {
      abort!(self.pos, "Expected {}.", fmt::AsDescription(expected));
    }

    let c = self.read_char()?;

    if *c != expected {
      abort!(
        c.span,
        "Expected {}, found {}.",
        fmt::AsDescription(expected),
        fmt::AsDescription(*c)
      );
    }

    Ok(c)
  }

  /// Reads a comment from the input.
  fn read_comment(&mut self) -> Result<()> {
    self.read_str_exact("//")?;

    while !self.is_eof() {
      if *self.read_char()? == '\n' {
        break;
      }
    }

    Ok(())
  }

  /// Reads a multiline comment from the input.
  fn read_comment_multiline(&mut self) -> Result<()> {
    self.read_str_exact("/*")?;

    while !self.is_eof() && !self.peek_str_exact("*/") {
      self.read_char()?;
    }

    self.read_str_exact("*/")
  }

  /// Reads an end delimiter from the input.
  fn read_delimiter(&mut self) -> Result<()> {
    let c = self.read_char()?;

    match *c {
      '(' | '[' | '{' => {
        let delim = syn::Delimiter::new(c.span, *c);

        self.add_token(delim.clone());
        self.open_delims.push(delim);
      }

      ')' | ']' | '}' => {
        let expected = self.open_delims.pop().map(|d| d.as_rev_char());

        match expected {
          Some(e) if e == *c => {
            self.add_token(syn::Delimiter::new(c.span, *c));
          }

          Some(expected) => {
            abort!(
              c.span,
              "Expected {}, found {}.",
              fmt::AsDescription(expected),
              fmt::AsDescription(*c)
            );
          }

          None => {
            abort!(c.span, "Unexpected {}.", fmt::AsDescription(*c));
          }
        }
      }

      other => abort!(c.span, "Unexpected {}.", fmt::AsDescription(other)),
    }

    Ok(())
  }

  /// Reads a number token from the input.
  fn read_number(&mut self) -> Result<(), Error> {
    let start_pos = self.pos;
    let mut is_float = false;

    // Read an integer or floating-point value.

    if self.peek_char_exact('.') {
      // Read a number that starts with `.`.

      self.read_number_decimal()?;

      is_float = true;
    } else {
      // Read a number that starts with a digit and may have a decimal suffix.

      self.read_number_digits()?;

      if self.peek_number_decimal() {
        self.read_number_decimal()?;

        is_float = true;
      }
    }

    // Read an optional exponent part.

    if self.peek_char_exact('e') || self.peek_char_exact('E') {
      self.read_char()?;
      self.read_number_digits()?;

      is_float = true;
    }

    // Parse the input source.

    let source = self.source(start_pos..self.pos);

    match is_float {
      true => match source.parse() {
        Ok(value) => self.add_token(syn::Number::from(syn::Float::new(source.span, value))),
        Err(err) => abort!(source.span, "Failed to parse floating point value. {}", err),
      },

      false => match source.parse() {
        Ok(value) => self.add_token(syn::Number::from(syn::Integer::new(source.span, value))),
        Err(err) => abort!(source.span, "Failed to parse integer value. {}", err),
      },
    }

    Ok(())
  }

  /// Reads a decimal point and one or more digits from the input.
  fn read_number_decimal(&mut self) -> Result<(), Error> {
    self.read_char_exact('.')?;
    self.read_number_digits()?;

    Ok(())
  }

  /// Reads one or more digits from the input.
  fn read_number_digits(&mut self) -> Result<(), Error> {
    // Read the first digit.

    let c = self.read_char()?;

    if !c.is_ascii_digit() {
      abort!(c.span, "Expected digit, found {}.", fmt::AsDescription(*c));
    }

    // Read the remaining digits.

    while matches!(self.peek_char(), Some(c) if c.is_ascii_digit()) {
      self.read_char()?;
    }

    Ok(())
  }

  /// Reads a string token from the input.
  fn read_string(&mut self) -> Result<(), Error> {
    let start_pos = self.pos;

    // Read the starting deiimiter of the string.

    let delim = self.read_char()?;

    if !matches!(*delim, '"' | '\'') {
      abort!(delim.span, "Unexpected {}.", fmt::AsDescription(*delim));
    }

    // Read the contents of the string.

    let mut contents = String::new();

    while matches!(self.peek_char(), Some(c) if c != *delim) {
      let c = self.read_char()?;

      if *c == '\\' {
        self.read_string_escape(&mut contents);
      } else {
        contents.push(*c);
      }
    }

    // Read the end delimiter.

    self.read_char_exact(*delim)?;

    // Add the token.

    self.add_token(syn::StringLiteral::new(start_pos..self.pos, contents));

    Ok(())
  }

  /// Reads a string escape sequence from the input and writes its value to the
  /// output.
  fn read_string_escape(&mut self, output: &mut String) {
    if self.is_eof() {
      return;
    }

    let c = self.read_char().expect("unexpected read error");

    match *c {
      'n' => output.push('\n'),
      'r' => output.push('\r'),

      '\\' | '\'' | '"' => output.push(*c),

      _ => {
        self.errors.add(err!(c.span, "Unknown string escape {}.", fmt::AsDescription(*c)));
      }
    }
  }

  /// Reads a literal string of characters from the input.
  fn read_str_exact(&mut self, literal: &str) -> Result<(), Error> {
    if !self.peek_str_exact(literal) {
      abort!(self.pos, "Expected `{}`.", literal.escape_debug());
    }

    self.pos.advance_str(literal);

    Ok(())
  }

  /// Reads a symbol token from the input.
  fn read_symbol_token(&mut self) -> Result<(), Error> {
    let c = self.read_char()?;

    match *c {
      '!' | '%' | '&' | '*' | ',' | '.' | '/' | '\\' | ':' | ';' | '<' | '>' | '=' | '?' | '^'
      | '+' | '-' => {
        self.add_token(syn::Symbol::new(c.span, *c));
      }

      _ => {
        abort!(c.span, "Unexpected {}.", fmt::AsDescription(*c));
      }
    };

    Ok(())
  }

  /// Reads a word token from the input.
  fn read_word_token(&mut self) -> Result<(), Error> {
    let start_pos = self.pos;

    // Read the start character.

    let c = self.read_char()?;

    if !syn::Word::is_start_char(*c) {
      abort!(c.span, "Unexpected {}.", fmt::AsDescription(*c));
    }

    // Read zero or more continue characters.

    while matches!(self.peek_char(), Some(c) if syn::Word::is_continue_char(c)) {
      self.read_char()?;
    }

    // Create a token from the input source.

    let source = self.source(start_pos..self.pos);

    self.add_token(syn::Word::new(source.span, *source));

    Ok(())
  }

  /// Returns the remaining input text.
  fn remaining(&self) -> &'src str {
    &self.input[self.pos.byte()..]
  }

  /// Returns the a span of input source.
  fn source(&self, span: impl Into<Span>) -> Spanned<&'src str> {
    let span = span.into();

    Spanned::new(span, &self.input[span.byte_range()])
  }
}

impl<T> Spanned<T> {
  fn new(span: impl Into<Span>, value: T) -> Self {
    Self { span: span.into(), value }
  }
}
