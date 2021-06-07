use crate::{Environment, ManagedKernelParameter};
use std::convert::TryInto;

/// An interface to manage a set of kernel parameters.
#[derive(Debug)]
pub struct KernelParameterManager {
  pub managed: Vec<ManagedKernelParameter>,
}

impl KernelParameterManager {
  pub fn new(managed: Vec<ManagedKernelParameter>) -> Self {
    Self { managed }
  }

  /// Confirms that all managed parameters match their target values.
  /// Returns the number of failures.
  pub fn audit(&self) -> i32 {
    self
      .managed
      .iter()
      .fold(0, |failures, parameter| match parameter.check() {
        Ok(success) => {
          if success {
            failures
          } else {
            failures + 1
          }
        }
        Err(error) => {
          log::error!("{:?}", &error);
          failures + 1
        }
      })
  }

  /// Sets all managed parameters to their target values.
  /// Returns the number of failures.
  pub fn enforce(&self) -> i32 {
    self
      .managed
      .iter()
      .fold(0, |failures, parameter| match parameter.set() {
        Ok(_) => failures,
        Err(_) => failures + 1,
      })
  }

  /// Dumps internal state, for diagnostic/debugging purposes.
  pub(crate) fn dump(&self) -> i32 {
    println!("{:#?}", &self);
    0
  }
}

impl<T> From<Vec<T>> for KernelParameterManager
where
  T: TryInto<ManagedKernelParameter>,
{
  fn from(values: Vec<T>) -> Self {
    let mut managed: Vec<ManagedKernelParameter> = vec![];

    for value in values {
      match value.try_into() {
        Ok(parameter) => managed.push(parameter),
        Err(_) => {}
      }
    }

    Self::new(managed)
  }
}

impl From<Environment> for KernelParameterManager {
  fn from(environment: Environment) -> Self {
    let variables = environment.variables();
    Self::from(variables)
  }
}
