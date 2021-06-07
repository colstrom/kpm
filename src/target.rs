use crate::Rule;

/// Our expectations for a value.
#[derive(Debug)]
pub struct Target {
  pub rule: Rule,
  pub value: String,
}

impl Target {
  pub fn new<R: Into<Rule>, V: ToString>(rule: R, value: V) -> Self {
    let rule = rule.into();
    let value = value.to_string();

    Self { rule, value }
  }

  pub fn check<T: AsRef<str>>(&self, value: T) -> bool {
    let value = value.as_ref();
    if self.rule == Rule::EqualTo {
      self.value.eq(value)
    } else {
      self.check_as_isize(value)
    }
  }

  fn check_as_isize<T: AsRef<str>>(&self, value: T) -> bool {
    match self.value.parse::<isize>() {
      Err(error) => {
        log::error!("cannot parse target as integer: {:?}", &error);
        false
      }
      Ok(target) => match value.as_ref().parse::<isize>() {
        Err(error) => {
          log::error!("cannot parse value as integer: {:?}", &error);
          false
        }
        Ok(value) => self.rule.check(&value, &target),
      },
    }
  }
}

impl<T: ToString> From<T> for Target {
  fn from(value: T) -> Self {
    let value = value.to_string();
    let rule = Rule::from(&value);
    let prefix = rule.prefix();

    match value.strip_prefix(prefix) {
      Some(value) => Self::new(rule, value),
      None => Self::new(rule, value),
    }
  }
}
