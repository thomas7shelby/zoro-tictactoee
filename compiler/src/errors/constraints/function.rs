use crate::errors::{BooleanError, ExpressionError, FieldError, GroupError, StatementError, ValueError};
use leo_types::{Error as FormattedError, IntegerError, Span};
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("{}", _0)]
    BooleanError(#[from] BooleanError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FieldError(#[from] FieldError),

    #[error("{}", _0)]
    GroupError(#[from] GroupError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    StatementError(#[from] StatementError),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),
}

impl FunctionError {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            FunctionError::BooleanError(error) => error.set_path(path),
            FunctionError::ExpressionError(error) => error.set_path(path),
            FunctionError::Error(error) => error.set_path(path),
            FunctionError::FieldError(error) => error.set_path(path),
            FunctionError::GroupError(error) => error.set_path(path),
            FunctionError::IntegerError(error) => error.set_path(path),
            FunctionError::StatementError(error) => error.set_path(path),
            FunctionError::ValueError(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        FunctionError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn arguments_length(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!("function expected {} inputs, found {} inputs", expected, actual);

        Self::new_from_span(message, span)
    }

    pub fn invalid_array(actual: String, span: Span) -> Self {
        let message = format!("Expected function input array, found `{}`", actual);

        Self::new_from_span(message, span)
    }
}
