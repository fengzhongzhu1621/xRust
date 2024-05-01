//! [`std::process::Command`] customized for testing.

use bstr;
use std::ffi;
use std::io;
use std::io::{Read, Write};
use std::ops::Deref;
use std::path;
use std::process;

use crate::cmd::assert::Assert;
use crate::cmd::assert::OutputAssertExt;
use crate::cmd::outputs::OutputError;
use crate::cmd::outputs::OutputOkExt;
use crate::cmd::outputs::OutputResult;
use crate::str::{DebugBuffer, DebugBytes};

/// [`std::process::Command`] customized for testing.
#[derive(Debug)]
pub struct Command {
    cmd: process::Command,
    stdin: Option<bstr::BString>,
    timeout: Option<std::time::Duration>,
}

/// 构造函数
impl Command {
    /// Constructs a new `Command` from a `std` `Command`.
    pub fn from_std(cmd: process::Command) -> Self {
        Self { cmd, stdin: None, timeout: None }
    }

    /// Constructs a new `Command` for launching the program at
    pub fn new<S: AsRef<ffi::OsStr>>(program: S) -> Self {
        let cmd = process::Command::new(program);
        Self::from_std(cmd)
    }

    /// If `input`, write it to `child`'s stdin while also reading `child`'s
    /// stdout and stderr, then wait on `child` and return its status and output.
    ///
    /// This was lifted from `std::process::Child::wait_with_output` and modified
    /// to also write to stdin.
    fn wait_with_input_output(
        mut child: process::Child,
        input: Option<Vec<u8>>,
        timeout: Option<std::time::Duration>,
    ) -> io::Result<process::Output> {
        #![allow(clippy::unwrap_used)] // changes behavior in some tests

        fn read<R>(
            mut input: R,
        ) -> std::thread::JoinHandle<io::Result<Vec<u8>>>
        where
            R: Read + Send + 'static,
        {
            std::thread::spawn(move || {
                let mut ret = Vec::new();
                // 从输出流读取数据，并返回读取的数据
                input.read_to_end(&mut ret).map(|_| ret)
            })
        }

        // 启动一个线程将 input 写入到标准输入中
        let stdin = input.and_then(|i| {
            child.stdin.take().map(|mut stdin| {
                std::thread::spawn(move || stdin.write_all(&i))
            })
        });
        // 启动一个线程等待读取标准输出的结果
        let stdout = child.stdout.take().map(read);
        // 启动一个线程等待读取错误输出的结果
        let stderr = child.stderr.take().map(read);

        // 等待标准输入写入完成
        // Finish writing stdin before waiting, because waiting drops stdin.
        stdin.and_then(|t| t.join().unwrap().ok());

        let status = if let Some(timeout) = timeout {
            // 处理超时
            wait_timeout::ChildExt::wait_timeout(&mut child, timeout)
                .transpose()
                .unwrap_or_else(|| {
                    let _ = child.kill(); // 超时杀掉进程
                    child.wait()
                })
        } else {
            // 等待进程执行完成
            child.wait()
        }?;

        // 等待读取输出完成
        let stdout =
            stdout.and_then(|t| t.join().unwrap().ok()).unwrap_or_default();
        let stderr =
            stderr.and_then(|t| t.join().unwrap().ok()).unwrap_or_default();

        Ok(process::Output { status, stdout, stderr })
    }
}

impl Command {
    /// Write `buffer` to `stdin` when the `Command` is run.
    pub fn write_stdin<S>(&mut self, buffer: S) -> &mut Self
    where
        S: Into<Vec<u8>>,
    {
        self.stdin = Some(bstr::BString::from(buffer.into()));
        self
    }

    /// Error out if a timeout is reached
    pub fn timeout(&mut self, timeout: std::time::Duration) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    /// 设置命令的输入
    /// Write `path`s content to `stdin` when the `Command` is run.
    pub fn pipe_stdin<P>(&mut self, file: P) -> io::Result<&mut Self>
    where
        P: AsRef<path::Path>,
    {
        // 读文件内容
        let buffer = std::fs::read(file)?;
        // 设置文件内容到成员变量中
        Ok(self.write_stdin(buffer))
    }

    /// Run a `Command`, returning an [`OutputResult`].
    pub fn ok(&mut self) -> OutputResult {
        OutputOkExt::ok(self)
    }

    /// Run a `Command`, unwrapping the [`OutputResult`].

    pub fn unwrap(&mut self) -> process::Output {
        OutputOkExt::unwrap(self)
    }

    /// Run a `Command`, unwrapping the error in the [`OutputResult`].
    pub fn unwrap_err(&mut self) -> OutputError {
        OutputOkExt::unwrap_err(self)
    }

    /// Run a `Command` and make assertions on the [`Output`].
    pub fn assert(&mut self) -> Assert {
        OutputAssertExt::assert(self)
    }
}

/// Mirror [`std::process::Command`]'s API
impl Command {
    /// Adds an argument to pass to the program.
    ///
    /// Only one argument can be passed per use. So instead of:
    ///
    /// ```no_run
    /// # assert_cmd::Command::new("sh")
    /// .arg("-C /path/to/repo")
    /// # ;
    /// ```
    ///
    /// usage would be:
    ///
    /// ```no_run
    /// # assert_cmd::Command::new("sh")
    /// .arg("-C")
    /// .arg("/path/to/repo")
    /// # ;
    /// ```
    ///
    /// To pass multiple arguments see [`args`].
    ///
    /// [`args`]: Command::args()
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    ///
    /// Command::new("ls")
    ///         .arg("-l")
    ///         .arg("-a")
    ///         .unwrap();
    /// ```
    pub fn arg<S: AsRef<ffi::OsStr>>(&mut self, arg: S) -> &mut Self {
        self.cmd.arg(arg);
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// To pass a single argument see [`arg`].
    ///
    /// [`arg`]: Command::arg()
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    ///
    /// Command::new("ls")
    ///         .args(&["-l", "-a"])
    ///         .unwrap();
    /// ```
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<ffi::OsStr>,
    {
        self.cmd.args(args);
        self
    }

    /// Inserts or updates an environment variable mapping.
    ///
    /// Note that environment variable names are case-insensitive (but case-preserving) on Windows,
    /// and case-sensitive on all other platforms.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    ///
    /// Command::new("ls")
    ///         .env("PATH", "/bin")
    ///         .unwrap_err();
    /// ```
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<ffi::OsStr>,
        V: AsRef<ffi::OsStr>,
    {
        self.cmd.env(key, val);
        self
    }

    /// Adds or updates multiple environment variable mappings.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    /// use std::process::Stdio;
    /// use std::env;
    /// use std::collections::HashMap;
    ///
    /// let filtered_env : HashMap<String, String> =
    ///     env::vars().filter(|&(ref k, _)|
    ///         k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH"
    ///     ).collect();
    ///
    /// Command::new("printenv")
    ///         .env_clear()
    ///         .envs(&filtered_env)
    ///         .unwrap();
    /// ```
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<ffi::OsStr>,
        V: AsRef<ffi::OsStr>,
    {
        self.cmd.envs(vars);
        self
    }

    /// Removes an environment variable mapping.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    ///
    /// Command::new("ls")
    ///         .env_remove("PATH")
    ///         .unwrap_err();
    /// ```
    pub fn env_remove<K: AsRef<ffi::OsStr>>(&mut self, key: K) -> &mut Self {
        self.cmd.env_remove(key);
        self
    }

    /// Clears the entire environment map for the child process.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    ///
    /// Command::new("ls")
    ///         .env_clear()
    ///         .unwrap_err();
    /// ```
    pub fn env_clear(&mut self) -> &mut Self {
        self.cmd.env_clear();
        self
    }

    /// Sets the working directory for the child process.
    ///
    /// # Platform-specific behavior
    ///
    /// If the program path is relative (e.g., `"./script.sh"`), it's ambiguous
    /// whether it should be interpreted relative to the parent's working
    /// directory or relative to `current_dir`. The behavior in this case is
    /// platform specific and unstable, and it's recommended to use
    /// [`canonicalize`] to get an absolute program path instead.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use assert_cmd::Command;
    ///
    /// Command::new("ls")
    ///         .current_dir("/bin")
    ///         .unwrap();
    /// ```
    ///
    /// [`canonicalize`]: std::fs::canonicalize()
    pub fn current_dir<P: AsRef<path::Path>>(&mut self, dir: P) -> &mut Self {
        self.cmd.current_dir(dir);
        self
    }

    /// Executes the `Command` as a child process, waiting for it to finish and collecting all of its
    /// output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the resulting output).
    /// Stdin is not inherited from the parent and any attempt by the child process to read from
    /// the stdin stream will result in the stream immediately closing.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use assert_cmd::Command;
    /// use std::io::{self, Write};
    /// let output = Command::new("/bin/cat")
    ///                      .arg("file.txt")
    ///                      .output()
    ///                      .expect("failed to execute process");
    ///
    /// println!("status: {}", output.status);
    /// io::stdout().write_all(&output.stdout).unwrap();
    /// io::stderr().write_all(&output.stderr).unwrap();
    ///
    /// assert!(output.status.success());
    /// ```
    pub fn output(&mut self) -> io::Result<process::Output> {
        let spawn = self.spawn()?;
        Self::wait_with_input_output(
            spawn,
            self.stdin.as_deref().cloned(),
            self.timeout,
        )
    }

    fn spawn(&mut self) -> io::Result<process::Child> {
        // stdout/stderr should only be piped for `output` according to `process::Command::new`.
        self.cmd.stdin(process::Stdio::piped());
        self.cmd.stdout(process::Stdio::piped());
        self.cmd.stderr(process::Stdio::piped());

        self.cmd.spawn()
    }
}

impl From<process::Command> for Command {
    fn from(cmd: process::Command) -> Self {
        Command::from_std(cmd)
    }
}

impl<'c> OutputOkExt for &'c mut Command {
    fn ok(self) -> OutputResult {
        let output = self.output().map_err(OutputError::with_cause)?;
        if output.status.success() {
            Ok(output)
        } else {
            let error =
                OutputError::new(output).set_cmd(format!("{:?}", self.cmd));
            let error = if let Some(stdin) = self.stdin.as_ref() {
                error.set_stdin(stdin.deref().clone())
            } else {
                error
            };
            Err(error)
        }
    }

    fn unwrap_err(self) -> OutputError {
        match self.ok() {
            Ok(output) => {
                if let Some(stdin) = self.stdin.as_ref() {
                    panic!(
                        "Completed successfully:\ncommand=`{:?}`\nstdin=```{}```\nstdout=```{}```",
                        self.cmd,
                        DebugBytes::new(stdin),
                        DebugBytes::new(&output.stdout)
                    )
                } else {
                    panic!(
                        "Completed successfully:\ncommand=`{:?}`\nstdout=```{}```",
                        self.cmd,
                        DebugBytes::new(&output.stdout)
                    )
                }
            }
            Err(err) => err,
        }
    }
}

impl<'c> OutputAssertExt for &'c mut Command {
    fn assert(self) -> Assert {
        let output = match self.output() {
            Ok(output) => output,
            Err(err) => {
                panic!("Failed to spawn {:?}: {}", self, err);
            }
        };
        let assert = Assert::new(output)
            .append_context("command", format!("{:?}", self.cmd));
        if let Some(stdin) = self.stdin.as_ref() {
            assert.append_context(
                "stdin",
                DebugBuffer::new(stdin.deref().clone()),
            )
        } else {
            assert
        }
    }
}
