// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use indigo_proc_macros::FromIdn;

use super::syn::DescribeElement;
use super::*;

/// A trait for types that can be parsed from IDN.
pub trait FromIdn: Sized {
  /// Reads a value of this type from a `idn::Reader`.
  fn from_idn(reader: &mut Reader) -> Result<Self>;
}

/// A trait for types that can be parsed from IDN.
///
/// Unlike `FromIdn`, implementations of this trait must not consume any input
/// or add any errors when parsing fails.
pub trait TryFromIdn: Sized {
  /// Tries to read a value of this type from a `idn::Reader` or returns
  /// `None`.
  ///
  /// Unlike `FromIdn::from_idn`, this function does not consume any input or
  /// add any errors when parsing fails.
  fn try_from_idn(reader: &mut Reader) -> Option<Self>;
}

// Implement `FromIdn` for common types.

macro_rules! impl_for_element {
  ($name:ident, $desc:expr) => {
    impl FromIdn for syn::$name {
      fn from_idn(reader: &mut Reader) -> Result<Self> {
        let el: Option<syn::Element> = reader.try_read();

        match el {
          None => abort!(reader.span(), "Expected {}.", $desc),
          Some(syn::Element::$name(el)) => Ok(el),
          Some(other) => {
            abort!(other.span(), "Expected {}, found {}.", $desc, DescribeElement(&other))
          }
        }
      }
    }
  };
}

impl_for_element!(Group, "group");
impl_for_element!(Number, "number");
impl_for_element!(StringLiteral, "string");
impl_for_element!(Symbol, "symbol");
impl_for_element!(Word, "word");

impl FromIdn for () {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    reader.read_group('(')?.contents.finish();

    Ok(())
  }
}

impl FromIdn for Arc<str> {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    match reader.try_read() {
      Some(syn::Element::StringLiteral(string)) => Ok(string.into()),
      Some(syn::Element::Word(word)) => Ok(word.into()),

      Some(other) => {
        abort!(other.span(), "Expected string or word, found {}.", DescribeElement(&other))
      }

      None => abort!(reader.span(), "Expected string or word."),
    }
  }
}

impl FromIdn for String {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let string: Arc<str> = reader.read()?;

    Ok((*string).into())
  }
}

impl FromIdn for f64 {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let prefix = reader.try_read_symbol("+-");
    let mut value = reader.read::<syn::Float>()?.as_f64();

    if matches!(prefix, Some(p) if p.as_char() == '-') {
      value = -value;
    }

    Ok(value)
  }
}

impl FromIdn for f32 {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    Ok(reader.read::<f64>()? as f32)
  }
}

macro_rules! impl_for_uint {
  ($ty:ident) => {
    impl FromIdn for $ty {
      fn from_idn(reader: &mut Reader) -> Result<Self> {
        let prefix = reader.try_read_symbol("+-");
        let integer: syn::Integer = reader.read()?;

        let value = integer.as_u64();
        let mut span = integer.span();

        if let Some(p) = prefix {
          if p.as_char() == '-' {
            abort!(p.span(), "Expected non-negative integer.");
          }

          span += p.span();
        }

        if value > $ty::MAX as u64 {
          abort!(span, "Expected integer less than or equal to {}.", $ty::MAX)
        }

        Ok(value as $ty)
      }
    }
  };
}

impl_for_uint!(u64);
impl_for_uint!(u32);
impl_for_uint!(u16);
impl_for_uint!(u8);

macro_rules! impl_for_int {
  ($ty:ident) => {
    impl FromIdn for $ty {
      fn from_idn(reader: &mut Reader) -> Result<Self> {
        let prefix = reader.try_read_symbol("+-");
        let integer: syn::Integer = reader.read()?;

        let value = integer.as_u64();
        let mut is_neg = false;
        let mut span = integer.span();

        if let Some(p) = prefix {
          is_neg = p.as_char() == '-';
          span += p.span();
        }

        match is_neg {
          false if value > $ty::MAX as u64 => {
            abort!(span, "Expected integer less than or equal to {}.", $ty::MAX)
          }

          false => Ok(value as $ty),

          true if value > $ty::MAX as u64 + 1 => {
            abort!(span, "Expected integer greater than or equal to {}.", $ty::MIN)
          }

          true if value > $ty::MAX as u64 => Ok($ty::MIN),
          true => Ok(-(value as $ty)),
        }
      }
    }
  };
}

impl_for_int!(i64);
impl_for_int!(i32);
impl_for_int!(i16);
impl_for_int!(i8);

impl FromIdn for bool {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let el = reader.try_read::<syn::Element>();

    match el {
      None => abort!(reader.span(), "Expected `true` or `false`."),

      Some(syn::Element::Word(word)) if word.as_str() == "true" => Ok(true),
      Some(syn::Element::Word(word)) if word.as_str() == "false" => Ok(false),

      Some(other) => {
        abort!(other.span(), "Expected `true` or `false`, found {}.", DescribeElement(&other))
      }
    }
  }
}

impl<T: FromIdn> FromIdn for Option<T> {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    if reader.try_read_word("none").is_some() {
      return Ok(None);
    }

    Ok(Some(reader.read()?))
  }
}

macro_rules! impl_for_tuple {
  ($name:ident $($names:ident)+) => {
    impl<$($names,)+ $name> FromIdn for ($($names,)+ $name)
    where
      $($names: FromIdn,)+
      $name: FromIdn,
    {
      #[allow(non_snake_case)]
      fn from_idn(reader: &mut Reader) -> Result<Self> {
        let group = match reader.try_read::<syn::Element>() {
          Some(syn::Element::Group(g)) if g.open.as_char() == '(' => g,
          Some(other) => abort!(other.span(), "Expected tuple, found {}.", DescribeElement(&other)),
          None => abort!(reader.span(), "Expected tuple."),
        };

        let mut contents = Reader::with_context(reader.ctx().clone(), group.contents);
        let mut list = contents.read_list();

        $(
          let $names = match list.read_next()? {
            Some(x) => x,
            None => abort!(list.span(), "Expected an element.")
          };
        )+

        let $name = match list.read_next()? {
          Some(x) => x,
          None => abort!(list.span(), "Expected an element.")
        };

        list.finish();

        Ok(($($names),+, $name))
      }
    }
  };
}

macro_rules! impl_for_tuples {
  ($x:ident) => {};

  ($name:ident $($names:ident)+) => {
    impl_for_tuple!($name $($names)+);
    impl_for_tuples!($($names)+);
  };
}

impl_for_tuples!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

impl<T> FromIdn for Vec<T>
where
  T: FromIdn,
{
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let mut group = reader.read_group('[')?;
    let mut list = group.contents.read_list();
    let mut items = Vec::new();

    while let Some(item) = list.read_next()? {
      items.push(item);
    }

    Ok(items)
  }
}

impl<K, V> FromIdn for HashMap<K, V>
where
  K: Eq + FromIdn + Hash,
  V: FromIdn,
{
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let mut group = reader.read_group('{')?;
    let mut list = group.contents.read_list();
    let mut map = HashMap::new();

    while let Some(mut reader) = list.next() {
      let key = reader.read()?;
      reader.read_symbol("=")?;
      let value = reader.read_to_end()?;

      map.insert(key, value);
    }

    Ok(map)
  }
}
