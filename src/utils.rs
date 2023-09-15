use std::fmt::Display;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};
use thiserror::Error;

pub struct FileUtils {}

impl FileUtils {
    pub fn parse_text_file<P, T>(file_path: P) -> Result<Vec<T>, FileUtilsError>
    where
        P: AsRef<Path>,
        T: FromStr,
        T::Err: Display,
    {
        let file = File::open(file_path)?;
        let buf_reader = BufReader::new(file);

        let contents = buf_reader
            .lines()
            .map(|line| {
                let line = line?;
                T::from_str(&line).map_err(|e| FileUtilsError::ConvertStringError(e.to_string()))
            })
            .collect::<Result<Vec<T>, FileUtilsError>>()?;

        Ok(contents)
    }
}

#[derive(Debug, Error)]
pub enum FileUtilsError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("convert from str error {0}")]
    ConvertStringError(String),
}
