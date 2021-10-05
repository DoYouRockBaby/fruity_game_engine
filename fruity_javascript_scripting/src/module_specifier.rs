// Copyright 2018-2021 strongly made by the Deno authors. All rights reserved. MIT license.

use crate::normalize_path::normalize_path;
use crate::error::JsError;
use std::env::current_dir;
use url::ParseError;
use url::Url;

pub const DUMMY_SPECIFIER: &str = "<unknown>";

/// Resolved module specifier
pub type ModuleSpecifier = Url;

/// Resolves module using this algorithm:
/// <https://html.spec.whatwg.org/multipage/webappapis.html#resolve-a-module-specifier>
pub fn resolve_import(
  specifier: &str,
  base: &str,
) -> Result<ModuleSpecifier, JsError> {
  let url = match Url::parse(specifier) {
    // 1. Apply the URL parser to specifier.
    //    If the result is not failure, return he result.
    Ok(url) => url,

    // 2. If specifier does not start with the character U+002F SOLIDUS (/),
    //    the two-character sequence U+002E FULL STOP, U+002F SOLIDUS (./),
    //    or the three-character sequence U+002E FULL STOP, U+002E FULL STOP,
    //    U+002F SOLIDUS (../), return failure.
    Err(ParseError::RelativeUrlWithoutBase)
      if !(specifier.starts_with('/')
        || specifier.starts_with("./")
        || specifier.starts_with("../")) =>
    {
      return Err(JsError::ImportModuleWithoutPrefix(specifier.to_string()));
    }

    // 3. Return the result of applying the URL parser to specifier with base
    //    URL as the base URL.
    Err(ParseError::RelativeUrlWithoutBase) => {
      let base = if base == DUMMY_SPECIFIER {
        // Handle <unknown> case, happening under e.g. repl.
        // Use CWD for such case.

        // Forcefully join base to current dir.
        // Otherwise, later joining in Url would be interpreted in
        // the parent directory (appending trailing slash does not work)
        let path = current_dir().unwrap().join(base);
        Url::from_file_path(path).unwrap()
      } else {
        match Url::parse(base) {
          Ok(url) => Ok(url),
          Err(_err) => Err(JsError::InvalidUrl(base.to_string())),
        }?
      };
      match base.join(specifier) {
        Ok(url) => Ok(url),
        Err(_err) => Err(JsError::InvalidUrl(specifier.to_string())),
      }?
    }

    // If parsing the specifier as a URL failed for a different reason than
    // it being relative, always return the original error. We don't want to
    // return `ImportPrefixMissing` or `InvalidBaseUrl` if the real
    // problem lies somewhere else.
    Err(_err) => return Err(JsError::InvalidUrl(DUMMY_SPECIFIER.to_string())),
  };

  Ok(url)
}

/// Converts a string representing an absolute URL into a ModuleSpecifier.
pub fn resolve_url(
  url_str: &str,
) -> Result<ModuleSpecifier, JsError> {
  match Url::parse(url_str) {
    Ok(url) => Ok(url),
    Err(_err) => Err(JsError::InvalidUrl(url_str.to_string())),
  }
}

/// Takes a string representing either an absolute URL or a file path,
/// as it may be passed to deno as a command line argument.
/// The string is interpreted as a URL if it starts with a valid URI scheme,
/// e.g. 'http:' or 'file:' or 'git+ssh:'. If not, it's interpreted as a
/// file path; if it is a relative path it's resolved relative to the current
/// working directory.
pub fn resolve_url_or_path(
  specifier: &str,
) -> Result<ModuleSpecifier, JsError> {
  if specifier_has_uri_scheme(specifier) {
    resolve_url(specifier)
  } else {
    resolve_path(specifier)
  }
}

/// Converts a string representing a relative or absolute path into a
/// ModuleSpecifier. A relative path is considered relative to the current
/// working directory.
pub fn resolve_path(
  path_str: &str,
) -> Result<ModuleSpecifier, JsError> {
  let path = current_dir().unwrap().join(path_str);
  let path = normalize_path(&path);

  match Url::from_file_path(path.clone()) {
    Ok(url) => Ok(url),
    Err(_err) => Err(JsError::InvalidUrl(path_str.to_string())),
  }
}

/// Returns true if the input string starts with a sequence of characters
/// that could be a valid URI scheme, like 'https:', 'git+ssh:' or 'data:'.
///
/// According to RFC 3986 (https://tools.ietf.org/html/rfc3986#section-3.1),
/// a valid scheme has the following format:
///   scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
///
/// We additionally require the scheme to be at least 2 characters long,
/// because otherwise a windows path like c:/foo would be treated as a URL,
/// while no schemes with a one-letter name actually exist.
fn specifier_has_uri_scheme(specifier: &str) -> bool {
  let mut chars = specifier.chars();
  let mut len = 0usize;
  // THe first character must be a letter.
  match chars.next() {
    Some(c) if c.is_ascii_alphabetic() => len += 1,
    _ => return false,
  }
  // Second and following characters must be either a letter, number,
  // plus sign, minus sign, or dot.
  loop {
    match chars.next() {
      Some(c) if c.is_ascii_alphanumeric() || "+-.".contains(c) => len += 1,
      Some(':') if len >= 2 => return true,
      _ => return false,
    }
  }
}