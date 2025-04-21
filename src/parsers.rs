use std::any::Any;

use calamine::{Data, DataType, ExcelDateTime};
use chrono::{Date, DateTime, Datelike, Local, Timelike, Utc};

pub mod icici;
mod sbi;

pub struct DataConverter {}

/// The common data handling methods. These will be shared between multiple parsers
impl DataConverter {
    /// Clean string from data object
    fn clean_data_string(data: &Data) -> String {
        match data.get_string() {
            None => String::new(),
            Some(value) => Self::clean_string(value),
        }
    }

    /// Create new string with null characters removed
    fn clean_string(value: &str) -> String {
        value.chars().filter(|c| !c.eq(&'\0')).collect()
    }

    /// Get integer value from Data string by parsing.
    fn get_int(data: &Data) -> Option<i64> {
        let copy = Self::clean_data_string(data);
        match copy.parse() {
            Ok(val) => return Some(val),
            Err(msg) => {
                println!("Error: {}", msg);
                return None;
            }
        }
    }

    fn get_date(data: &Data) {
        let copy = Self::clean_data_string(data);
        // let date = DateTime::
        // TODO: write logic to parse date string.
    }

    // TODO: Add other methods to convert string into different data types
}
