use env_logger::Builder;
use env_logger::Env;

const LOG_FILTER_ENVIRONMENT_VARIABLE: &str = "KPM_LOG_FILTER";
const LOG_FILTER_DEFAULT_VALUE: &str = "info";
const LOG_STYLE_ENVIRONMENT_VARIABLE: &str = "KPM_LOG_STYLE";
const LOG_STYLE_DEFAULT_VALUE: &str = "auto";

pub fn init() {
  Builder::from_env(
    Env::new()
      .filter_or(LOG_FILTER_ENVIRONMENT_VARIABLE, LOG_FILTER_DEFAULT_VALUE)
      .write_style_or(LOG_STYLE_ENVIRONMENT_VARIABLE, LOG_STYLE_DEFAULT_VALUE),
  )
  .format_timestamp(None)
  .init()
}
