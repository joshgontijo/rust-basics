mod myerr;

use myerr::AppResult;
use crate::myerr::AppError;
use std::fs::File;
use std::io;


fn main() {
    // let _ = custom_error().unwrap();
    let _ = io_error().unwrap();
}

fn custom_error() -> AppResult<u32> {
    Err(AppError::Custom("My custom error message".to_string()))
}

fn io_error() -> AppResult<File> {
    Ok(File::open("some-path")?)
}

