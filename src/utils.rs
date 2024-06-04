use serde::Deserialize;
use serde_json::Value;

use crate::error::AppError;

pub fn validate<T>(body: Value) -> Result<T, AppError>
where
    T: for<'a> Deserialize<'a>,
{
    let data: Result<T, serde_json::Error> = serde_json::from_value(body);
    let req: T = match data {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError::SerdeError(e));
        }
    };
    Ok(req)
}
