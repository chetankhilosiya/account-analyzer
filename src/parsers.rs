use calamine::{Data, DataType};

pub mod icici;
mod sbi;

pub struct DataConverter {}

impl DataConverter {
    fn get_int(data: &Data) -> i64 {
        if data.is_int() {
            return data.get_int().unwrap_or_default();
        }

        if data.is_string() {
            // TODO: implement logic to convert string into integer
            return 1;
        }

        return 1;
    }

    // TODO: Add other methods to convert string into different data types
}
