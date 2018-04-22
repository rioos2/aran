// Copyright (c) 2018 Rio Advancement Inc
//

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {

}

/*impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        match *self {
        }
    }
}*/

