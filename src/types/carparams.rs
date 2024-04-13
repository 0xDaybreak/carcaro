use crate::handle_errors::Error;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct CarParams {
    pub make: String,
    pub model: String,
    pub year: i32,
}

pub(crate) fn extract_car_params(params: HashMap<String, String>) -> Result<CarParams, Error> {
    if params.contains_key("make") && params.contains_key("model") && params.contains_key("year") {
        return Ok(CarParams {
            make: params.get("make").unwrap().to_string(),
            model: params.get("model").unwrap().to_string(),
            year: params
                .get("year")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParams)
}
