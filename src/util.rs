use anyhow::{Context, Result};
use std::env;
use std::error::Error;
use std::str::FromStr;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn get_env<T>(env_name: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + Error + 'static,
{
    let s = env::var(env_name).context(f!("no {env_name}"))?;
    s.parse().context(f!("{env_name}={s}"))
}

pub fn read_atomic_bool(v: &Arc<AtomicBool>) -> bool {
    v.load(Ordering::Relaxed)
}
