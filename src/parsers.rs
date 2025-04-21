use std::any::Any;

use calamine::Data;

pub mod icici;
mod sbi;

pub struct DataConverter {}

impl DataConverter {
    fn clean_string(value: &str) -> String {
        value.chars().filter(|c| !c.eq(&'\0')).collect()
    }

    fn get_int(data: &Data) -> Option<i64> {
        match data {
            Data::Int(value) => {
                // println!("Got integer value: {}", value);
                return Some(*value);
            }
            Data::String(value) => {
                // println!("Got string value: {:?}", value);
                let copy = Self::clean_string(value);
                match copy.parse() {
                    Ok(val) => return Some(val),
                    Err(msg) => {
                        println!("Error: {}", msg);
                        return None;
                    }
                }
            }
            _ => {
                println!("Got non integer value: {:?}", data.type_id());
                return None;
            }
        }
    }

    // TODO: Add other methods to convert string into different data types
}
