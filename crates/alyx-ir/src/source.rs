use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageSource {
    Path(PathBuf),
    Bytes(Vec<u8>),
    Url(String),
}
