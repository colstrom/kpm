use crate::Target;
use std::convert::TryFrom;
use sysctl::{Ctl, Sysctl, SysctlError};

/// An interface to a single kernel parameter with a desired state.
#[derive(Debug)]
pub struct ManagedKernelParameter {
  name: String,
  control: Ctl,
  pub target: Target,
}

impl ManagedKernelParameter {
  pub fn new<S: ToString, C: Into<Ctl>, T: Into<Target>>(name: S, control: C, target: T) -> Self {
    let name = name.to_string();
    let control = control.into();
    let target = target.into();

    Self {
      name,
      control,
      target,
    }
  }

  pub fn get(&self) -> Result<String, SysctlError> {
    log::trace!("attempt reading value of {:?}", &self.name);

    match self.control.value_string() {
      Ok(value) => {
        log::trace!("success reading value of {:?}: {:?}", &self.name, &value);
        Ok(value)
      }
      Err(error) => {
        log::error!("failure reading value of {:?}: {:?}", &self.name, &error);
        Err(error)
      }
    }
  }

  pub fn set(&self) -> Result<String, SysctlError> {
    log::trace!("attempt writing value of {:?}", &self.name);

    match self.control.set_value_string(&self.target.value) {
      Ok(value) => {
        log::trace!("success writing value of {:?}: {:?}", &self.name, &value);
        Ok(value)
      }
      Err(error) => {
        log::error!("failure writing value of {:?}: {:?}", &self.name, &error);
        Err(error)
      }
    }
  }

  pub fn check(&self) -> Result<bool, SysctlError> {
    let value = self.get()?;

    if self.target.check(&value) {
      log::info!("{:?} matches {:?}", &self.name, &self.target);
      Ok(true)
    } else {
      log::warn!(
        "{:?} has value: {:?}, does not match {:?}",
        &self.name,
        &value,
        &self.target,
      );
      Ok(false)
    }
  }
}

impl<S, T> TryFrom<(S, T)> for ManagedKernelParameter
where
  S: AsRef<str>,
  T: Into<Target>,
{
  type Error = SysctlError;

  fn try_from((name, target): (S, T)) -> Result<Self, Self::Error> {
    let control = Ctl::new(name.as_ref())?;
    let name = control.name()?;
    let target = target.into();
    Ok(Self::new(name, control, target))
  }
}
