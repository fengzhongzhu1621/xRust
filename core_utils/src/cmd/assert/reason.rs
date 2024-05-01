use super::tree::CaseTree;

#[derive(Debug)]
pub(crate) enum AssertReason {
    UnexpectedFailure { actual_code: Option<i32> },
    UnexpectedSuccess,
    UnexpectedCompletion,
    CommandInterrupted,
    UnexpectedReturnCode { case_tree: CaseTree },
    UnexpectedStdout { case_tree: CaseTree },
    UnexpectedStderr { case_tree: CaseTree },
}
