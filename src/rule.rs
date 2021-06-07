const PREFIX_EQ: &str = "==";
const PREFIX_GE: &str = "=>";
const PREFIX_LE: &str = "=<";

/// How we decide if something is "correct".
#[derive(Debug, PartialEq)]
pub enum Rule {
  EqualTo,
  AtLeast,
  AtMost,
}

impl Default for Rule {
  fn default() -> Self {
    Self::EqualTo
  }
}

impl Rule {
  /// Checks if a value matches a target, according to the rule.
  pub fn check<T: PartialOrd>(&self, value: &T, target: &T) -> bool {
    match self {
      Self::EqualTo => value.eq(target),
      Self::AtLeast => value.ge(target),
      Self::AtMost => value.le(target),
    }
  }

  /// Returns the prefix for this rule.
  pub(crate) fn prefix(&self) -> &str {
    match self {
      Self::EqualTo => PREFIX_EQ,
      Self::AtLeast => PREFIX_GE,
      Self::AtMost => PREFIX_LE,
    }
  }
}

impl<T: AsRef<str>> From<T> for Rule {
  fn from(value: T) -> Rule {
    let value = value.as_ref();

    if value.starts_with(PREFIX_EQ) {
      Self::EqualTo
    } else if value.starts_with(PREFIX_GE) {
      Self::AtLeast
    } else if value.starts_with(PREFIX_LE) {
      Self::AtMost
    } else {
      Self::default()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn eq() {
    assert_eq!(Rule::from("=="), Rule::EqualTo)
  }

  #[test]
  fn ge() {
    assert_eq!(Rule::from("=>"), Rule::AtLeast)
  }

  #[test]
  fn le() {
    assert_eq!(Rule::from("=<"), Rule::AtMost)
  }
}
