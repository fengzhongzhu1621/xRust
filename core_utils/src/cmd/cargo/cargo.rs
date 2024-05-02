use super::error::{CargoError, NotFoundError};
use std::{self, env, path, process};

pub(crate) fn cargo_runner() -> Option<Vec<String>> {
    let runner_env = format!(
        "CARGO_TARGET_{}_RUNNER",
        CURRENT_TARGET.replace('-', "_").to_uppercase()
    );
    let runner = env::var(runner_env).ok()?;
    Some(runner.split(' ').map(str::to_string).collect())
}

/// 获得当前二进制执行文件的路径
/// Look up the path to a cargo-built binary within an integration test.
pub fn cargo_bin<S: AsRef<str>>(name: S) -> path::PathBuf {
    cargo_bin_str(name.as_ref())
}

/// 获得当前二进制执行文件的路径
fn cargo_bin_str(name: &str) -> path::PathBuf {
    // 获得当前二进制执行文件的路径
    let env_var = format!("CARGO_BIN_EXE_{}", name);
    env::var_os(env_var).map(|p| p.into()).unwrap_or_else(|| {
        // 获得二进制文件所在的目录 target/debug/{name}.{suffix}
        target_dir().join(format!("{}{}", name, env::consts::EXE_SUFFIX))
    })
}

/// 获得二进制文件所在的目录 target/debug/
// Adapted from
// https://github.com/rust-lang/cargo/blob/485670b3983b52289a2f353d589c57fae2f60f82/tests/testsuite/support/mod.rs#L507
fn target_dir() -> path::PathBuf {
    env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .expect("this should only be used where a `current_exe` can be set")
}

pub(crate) fn cargo_bin_cmd<S: AsRef<str>>(
    name: S,
) -> Result<process::Command, CargoError> {
    // 获得当前二进制执行文件的路径
    let path = cargo_bin(name);
    if path.is_file() {
        if let Some(runner) = cargo_runner() {
            let mut cmd = process::Command::new(&runner[0]);
            cmd.args(&runner[1..]).arg(path);
            Ok(cmd)
        } else {
            Ok(process::Command::new(path))
        }
    } else {
        // 返回路径不存在错误
        Err(CargoError::with_cause(NotFoundError { path }))
    }
}

pub trait CommandCargoExt
where
    Self: Sized,
{
    /// Create a [`Command`] to run a specific binary of the current crate.
    fn cargo_bin<S: AsRef<str>>(name: S) -> Result<Self, CargoError>;
}

impl CommandCargoExt for crate::cmd::process::Command {
    fn cargo_bin<S: AsRef<str>>(name: S) -> Result<Self, CargoError> {
        crate::cmd::process::Command::cargo_bin(name)
    }
}

impl CommandCargoExt for process::Command {
    fn cargo_bin<S: AsRef<str>>(name: S) -> Result<Self, CargoError> {
        cargo_bin_cmd(name)
    }
}

/// The current process' target triplet.
const CURRENT_TARGET: &str =
    include_str!(concat!(env!("OUT_DIR"), "/current_target.txt"));
