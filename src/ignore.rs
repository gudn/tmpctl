#![allow(dead_code)]
use std::io;

use lazy_static::lazy_static;
use regex::bytes::RegexSet;

lazy_static! {
  static ref SEP: String = if std::path::MAIN_SEPARATOR == '\\' {
    "\\\\".into()
  } else {
    "/".into()
  };
  static ref QUESTION_MARK: String = format!("[^{}]", *SEP);
  static ref ASTERIK_MARK: String = format!("[^{}]*", *SEP);
  static ref DOUBLE_ASTERIK_MARK: String = ".*".into();
  static ref DOT_CHAR: String = "\\.".into();
}

fn process_char(c: char, chars: &mut std::str::Chars, out: &mut String) -> bool {
  match c {
    '/' => {
      out.push_str(SEP.as_str());
      true
    }
    '.' => {
      out.push_str(DOT_CHAR.as_str());
      false
    }
    '?' => {
      out.push_str(QUESTION_MARK.as_str());
      false
    }
    '*' => match chars.next() {
      Some('*') => {
        out.push_str(DOUBLE_ASTERIK_MARK.as_str());
        false
      }
      Some(c) => {
        out.push_str(ASTERIK_MARK.as_str());
        process_char(c, chars, out)
      }
      None => {
        out.push_str(ASTERIK_MARK.as_str());
        false
      }
    },
    _ => {
      out.push(c);
      false
    }
  }
}

fn parse_line(line: &str) -> String {
  if let Some(line) = line.strip_prefix(':') {
    return line.trim().to_string();
  }
  let mut out = String::with_capacity(line.len());
  let mut chars = line.chars();
  let mut slash_count = 0;
  let mut last_slash = false;
  while let Some(c) = chars.next() {
    if out.is_empty() && c == '/' {
      slash_count += 2;
      continue;
    }
    if !last_slash || c != '/' {
      if last_slash {
        out.push_str(r"\b");
      }
      last_slash = process_char(c, &mut chars, &mut out);
      if last_slash {
        slash_count += 1;
      }
    }
  }
  if last_slash {
    slash_count -= 1
  } else {
    out.push_str(SEP.as_str());
    out.push('?');
  }
  out.push('$');
  if slash_count > 0 {
    let mut new_out = String::with_capacity(out.len() + 1);
    new_out.push('^');
    new_out.push_str(&out);
    out = new_out
  } else {
    let mut new_out = String::with_capacity(out.len() + 1);
    new_out.push_str(r"(?:^|\b)");
    new_out.push_str(&out);
    out = new_out
  }
  out
}

pub(crate) fn parse_lines<B: io::BufRead>(lines_: io::Lines<B>) -> Result<RegexSet, crate::Error> {
  let mut lines = Vec::with_capacity(lines_.size_hint().1.unwrap_or_default());
  for l in lines_ {
    lines.push(l?);
  }
  let regexps: Vec<_> = lines
    .into_iter()
    .filter_map(|l| {
      let l = l.trim();
      if l.is_empty() {
        return None;
      }
      if let Some(l) = l.strip_prefix('\\') {
        Some(parse_line(l))
      } else if l.starts_with('#') {
        None
      } else {
        Some(parse_line(l))
      }
    })
    .collect();
  RegexSet::new(regexps).map_err(crate::Error::from)
}

#[cfg(test)]
mod tests {
  use std::io::BufRead;

  use super::*;

  static FILE: &str = r#"
# ignore this line

some/file
sub1/*/sub2
sub[34]/**/sub[34]
files/
s*.patt
*.py[cod]
"#;

  #[test]
  fn matching_files() -> Result<(), crate::Error> {
    let reader = io::Cursor::new(FILE.as_bytes());
    let reader = io::BufReader::new(reader);
    let set = parse_lines(reader.lines())?;
    assert!(!set.is_match(b"some/file/again"));
    assert!(set.is_match(b"some/file"));
    assert!(!set.is_match(b"not/some/file"));
    assert!(set.is_match(b"in/some/files/"));
    assert!(set.is_match(b"compiled.pyc"));
    assert!(set.is_match(b"in/some/sad.patt"));
    // FIXME
    assert!(!set.is_match(b"in/some/2sad.patt"));
    assert!(set.is_match(b"sub1/blabla/sub2"));
    assert!(!set.is_match(b"sub1/bla/bla/sub2"));
    assert!(set.is_match(b"sub3/blabla/sub4"));
    assert!(set.is_match(b"sub4/bla/bla/sub3"));
    Ok(())
  }
}
