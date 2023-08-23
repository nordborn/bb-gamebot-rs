use anyhow::{Context, Result};
use std::env;
use std::error::Error;
use std::str::FromStr;

pub fn get_env<T>(env_name: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + Error + 'static,
{
    let s = env::var(env_name).with_context(|| format!("no {}", env_name))?;
    s.parse().with_context(|| format!("{}={}", env_name, s))
}
