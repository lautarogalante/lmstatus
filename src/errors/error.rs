use crate::prelude::*;

pub enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::Io(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::Parse(error)
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::Io(err) => write!(f, "CliError::Io({:?})", err),
            CliError::Parse(err) => write!(f, "CliError::Parse({:?})", err),
        }
    }
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::Io(err) => write!(f, "CliError::Io({:?})", err),
            CliError::Parse(err) => write!(f, "CliError::Parse({:?})", err),
        }
    }
}
