use super::error::AssertError;
use super::reason::AssertReason;
use super::tree::CaseTree;
use crate::cmd::{
    color::Palette,
    outputs::output_fmt,
    predicate::{IntoCodePredicate, IntoOutputPredicate},
};
use predicates_core;
use predicates_tree::CaseTreeExt;
use std::fmt;
use std::process;

pub type AssertResult = Result<Assert, AssertError>;

/// Assert the state of an [`Output`].
///
/// Create an `Assert` through the [`OutputAssertExt`] trait.
pub struct Assert {
    pub(crate) output: process::Output, // 进程输出
    pub(crate) context:
        Vec<(&'static str, Box<dyn fmt::Display + Send + Sync>)>,
}

impl Assert {
    /// Create an `Assert` for a given [`Output`].
    ///
    /// [`Output`]: std::process::Output
    pub fn new(output: process::Output) -> Self {
        // 构造函数，上下文默认为空
        Self { output, context: vec![] }
    }

    fn into_error(self, reason: AssertReason) -> AssertError {
        AssertError { assert: self, reason }
    }

    pub fn append_context<D>(mut self, name: &'static str, context: D) -> Self
    where
        D: fmt::Display + Send + Sync + 'static,
    {
        // 使用 Box 的原因：类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
        self.context.push((name, Box::new(context)));
        self
    }

    /// Access the contained [`Output`].
    ///
    /// [`Output`]: std::process::Output
    pub fn get_output(&self) -> &process::Output {
        &self.output
    }

    #[track_caller]
    pub fn success(self) -> Self {
        self.try_success().unwrap_or_else(AssertError::panic)
    }

    /// `try_` variant of [`Assert::success`].
    pub fn try_success(self) -> AssertResult {
        if !self.output.status.success() {
            let actual_code = self.output.status.code();
            return Err(self
                .into_error(AssertReason::UnexpectedFailure { actual_code }));
        }
        Ok(self)
    }

    #[track_caller]
    pub fn failure(self) -> Self {
        self.try_failure().unwrap_or_else(AssertError::panic)
    }

    /// Variant of [`Assert::failure`] that returns an [`AssertResult`].
    pub fn try_failure(self) -> AssertResult {
        if self.output.status.success() {
            return Err(self.into_error(AssertReason::UnexpectedSuccess));
        }
        Ok(self)
    }

    /// Ensure the command aborted before returning a code.
    #[track_caller]
    pub fn interrupted(self) -> Self {
        self.try_interrupted().unwrap_or_else(AssertError::panic)
    }

    /// Variant of [`Assert::interrupted`] that returns an [`AssertResult`].
    pub fn try_interrupted(self) -> AssertResult {
        if self.output.status.code().is_some() {
            return Err(self.into_error(AssertReason::UnexpectedCompletion));
        }
        Ok(self)
    }

    #[track_caller]
    pub fn code<I, P>(self, pred: I) -> Self
    where
        I: IntoCodePredicate<P>, // 输入参数实现了方法 into_code()，其返回结果是 predicates_core::Predicate<i32> 类型
        P: predicates_core::Predicate<i32>,
    {
        self.try_code(pred).unwrap_or_else(AssertError::panic)
    }

    /// Variant of [`Assert::code`] that returns an [`AssertResult`].
    pub fn try_code<I, P>(self, pred: I) -> AssertResult
    where
        I: IntoCodePredicate<P>, // 实现了 into_code() 方法，返回类型实现了 predicates_core::Predicate<i32>
        P: predicates_core::Predicate<i32>,
    {
        self.code_impl(&pred.into_code())
    }

    fn code_impl(
        self,
        pred: &dyn predicates_core::Predicate<i32>,
    ) -> AssertResult {
        // 获得进程执行的返回码
        let actual_code = if let Some(actual_code) = self.output.status.code()
        {
            actual_code
        } else {
            return Err(self.into_error(AssertReason::CommandInterrupted));
        };
        // 如果断言不相同
        if let Some(case) = pred.find_case(false, &actual_code) {
            return Err(self.into_error(AssertReason::UnexpectedReturnCode {
                case_tree: CaseTree(case.tree()),
            }));
        }
        Ok(self)
    }

    #[track_caller]
    pub fn stdout<I, P>(self, pred: I) -> Self
    where
        I: IntoOutputPredicate<P>,
        P: predicates_core::Predicate<[u8]>,
    {
        self.try_stdout(pred).unwrap_or_else(AssertError::panic)
    }

    /// Variant of [`Assert::stdout`] that returns an [`AssertResult`].
    pub fn try_stdout<I, P>(self, pred: I) -> AssertResult
    where
        I: IntoOutputPredicate<P>,
        P: predicates_core::Predicate<[u8]>,
    {
        self.stdout_impl(&pred.into_output())
    }

    fn stdout_impl(
        self,
        pred: &dyn predicates_core::Predicate<[u8]>,
    ) -> AssertResult {
        {
            let actual = &self.output.stdout;
            if let Some(case) = pred.find_case(false, actual) {
                return Err(self.into_error(AssertReason::UnexpectedStdout {
                    case_tree: CaseTree(case.tree()),
                }));
            }
        }
        Ok(self)
    }

    #[track_caller]
    pub fn stderr<I, P>(self, pred: I) -> Self
    where
        I: IntoOutputPredicate<P>,
        P: predicates_core::Predicate<[u8]>,
    {
        self.try_stderr(pred).unwrap_or_else(AssertError::panic)
    }

    /// Variant of [`Assert::stderr`] that returns an [`AssertResult`].
    pub fn try_stderr<I, P>(self, pred: I) -> AssertResult
    where
        I: IntoOutputPredicate<P>,
        P: predicates_core::Predicate<[u8]>,
    {
        self.stderr_impl(&pred.into_output())
    }

    fn stderr_impl(
        self,
        pred: &dyn predicates_core::Predicate<[u8]>,
    ) -> AssertResult {
        {
            let actual = &self.output.stderr;
            if let Some(case) = pred.find_case(false, actual) {
                return Err(self.into_error(AssertReason::UnexpectedStderr {
                    case_tree: CaseTree(case.tree()),
                }));
            }
        }
        Ok(self)
    }
}

impl fmt::Display for Assert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let palette = Palette::color();
        // 打印上下文
        for (name, context) in &self.context {
            writeln!(
                f,
                "{:#}=`{:#}`",
                palette.key(name),
                palette.value(context)
            )?;
        }
        // 打印进程输出
        output_fmt(&self.output, f)
    }
}

impl fmt::Debug for Assert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Assert").field("output", &self.output).finish()
    }
}
