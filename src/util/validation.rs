use anyhow::Error;
use lazy_static::lazy_static;
use regex::Regex;

// pub fn error_hadler<T>(result: Result<T, Error>) {
//     match result {

//     }
// }

pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email.trim())
}
