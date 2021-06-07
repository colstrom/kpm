/// A subset of the program environment, where keys match a certain prefix.
#[derive(Debug)]
pub struct Environment {
  pub prefix: String,
}

impl Default for Environment {
  fn default() -> Self {
    Self {
      prefix: String::from("sysctl-"),
    }
  }
}

impl Environment {
  pub fn new<T: ToString>(prefix: T) -> Self {
    let prefix = prefix.to_string();
    Self { prefix }
  }

  /// Returns the matching environment variables, without the prefix.
  pub fn variables(&self) -> Vec<(String, String)> {
    log::info!("searching for variables prefixed with: {:?}", &self.prefix);

    std::env::vars()
      .filter_map(|(var, val)| match var.strip_prefix(&self.prefix) {
        Some(key) => {
          log::debug!("found {:?}", &var);
          Some((key.to_owned(), val))
        }
        None => {
          log::trace!("ignored: {:?}", &var);
          None
        }
      })
      .collect()
  }
}
