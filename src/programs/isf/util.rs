// a fork of https://github.com/nannou-org/nannou/blob/master/nannou_isf/src/pipeline.rs

use std::path::Path;
use thiserror::Error;

/// Errors that can occur while trying to load and parse ISF from the fragment shader.
#[derive(Debug, Error)]
pub enum IsfError {
    #[error("{err}")]
    Parse {
        #[from]
        err: isf::ParseError,
    },
    #[error("failed to read fragment shader for ISF: {err}")]
    Io {
        #[from]
        err: std::io::Error,
    },
}

pub fn read_isf_from_path(path: &Path) -> Result<isf::Isf, IsfError> {
    std::fs::read_to_string(path)
        .map_err(IsfError::from)
        .and_then(|s| isf::parse(&s).map_err(From::from))
}

pub fn split_result<T, E>(res: Result<T, E>) -> (Option<T>, Option<E>) {
    match res {
        Ok(t) => (Some(t), None),
        Err(e) => (None, Some(e)),
    }
}
