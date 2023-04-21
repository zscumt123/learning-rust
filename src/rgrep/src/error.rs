use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("regex pattern error")]
    RegexPatternError(#[from] regex::Error),
    #[error("glob parttern error")]
    GlobPartternError(#[from] glob::PatternError),
}
