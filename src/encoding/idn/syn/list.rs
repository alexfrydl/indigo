// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A helper for reading comma-separated IDN values.
pub struct ListReader<'r> {
  reader: &'r mut Reader,
}

impl<'a> ListReader<'a> {
  /// Constructs a new list iterator from a IDN reader.
  pub fn new(reader: &'a mut Reader) -> Self {
    Self { reader }
  }

  /// Returns the remaining span of the list.
  pub fn span(&self) -> Span {
    self.reader.span()
  }

  /// Finishes reading the list, adding a non-fatal error if there are extra
  /// elements.
  pub fn finish(&mut self) {
    self.reader.finish();
  }

  /// Reads a `Reader` for the next item in the list or returns `None` if no
  /// items remain.
  pub fn next(&mut self) -> Option<Reader> {
    if self.reader.is_empty() {
      return None;
    }

    let mut tokens = self.reader.tokens().clone();

    // Read elements for the next list item.

    while let Some(el) = self.reader.try_read() {
      // If the element is a comma, the item is terminated.

      if matches!(el, Element::Symbol(sym) if sym.as_char() == ',') {
        tokens.truncate_before(self.reader.tokens());
        tokens.pop();

        break;
      }

      // Or, if the next token is on a new line, the item is terminated
      // implicitly.

      let token = self.reader.tokens().peek();

      if matches!(token, Some(t) if t.span().start().line() > el.span().last_line()) {
        tokens.truncate_before(self.reader.tokens());

        break;
      }
    }

    Some(Reader::with_context(self.reader.ctx(), tokens))
  }

  /// Reads the next item in the list or returns `None` if no items remain.
  pub fn read_next<T: FromIdn>(&mut self) -> Result<Option<T>> {
    Ok(match self.next() {
      Some(mut item) => Some(item.read_to_end()?),
      None => None,
    })
  }
}

impl Reader {
  /// Returns a `ListReader` which can read comma-separated values.
  pub fn read_list(&mut self) -> ListReader {
    ListReader::new(self)
  }
}
