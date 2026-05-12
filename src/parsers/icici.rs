use anyhow::Result;
use std::{f64, path::Path};

use calamine::{Data, Range, Reader, Xls, open_workbook};

use crate::{
    db::{self, Amount},
    parsers::DataConverter,
};

pub struct ICICIParser {}

impl ICICIParser {
    pub fn new() -> Self {
        ICICIParser {}
    }

    /// The complete logic to parse xls file and create database of transactions.
    pub fn parse_file(&self, file: &Path) -> Result<db::DB> {
        let mut workbook: Xls<_> = open_workbook(file).expect("Can't open file");
        let mut db = db::DB::new(crate::Bank::ICICI);
        let sheets = workbook.worksheets();
        assert!(sheets.len() == 1, "Sheets are empty");
        let sheet = sheets.first().expect("First sheet should be present");
        let range = &sheet.1;
        if range.is_empty() {
            return Result::Ok(db);
        }

        let table_range = ICICIParser::get_table_range(&self, range);
        print!("table range: ({}, {})", table_range.0, table_range.1);

        let mut row = table_range.0;
        while row <= table_range.1 {
            let mut row_data = db::Entry::default();
            range.get((row, 1)).inspect(|&date| {
                let cleaned = DataConverter::clean_data_string(date);
                row_data.set_value_date(&cleaned, "%d/%m/%Y");
            });

            range.get((row, 2)).inspect(|&date| {
                // println!(
                //     "transaction date: {:?}",
                //     DataConverter::clean_data_string(date)
                // )
                let cleaned = DataConverter::clean_data_string(date);
                row_data.set_transaction_date(&cleaned, "%d/%m/%Y");
            });

            // 3rd column is cheque number which is not required

            range.get((row, 4)).inspect(|&details| {
                // println!(
                // "transaction details: {:?}",
                // DataConverter::clean_data_string(detail)
                // )
                let cleaned = DataConverter::clean_data_string(details);
                row_data.description = cleaned;
            });

            range.get((row, 5)).inspect(|&withdrawal_amount| {
                // println!(
                //     "withdrawal: {:?}",
                //     DataConverter::clean_data_string(withdrawal)
                // )
                let cleaned = DataConverter::clean_data_string(withdrawal_amount);
                let amount_value = cleaned.parse::<f64>().unwrap_or(0.0);
                if amount_value != 0.0 {
                    row_data.amount = Amount::new_withdrawal(amount_value);
                }
            });

            range.get((row, 6)).inspect(|&deposite_amount| {
                // println!("deposite: {:?}", DataConverter::clean_data_string(deposite))
                let cleaned = DataConverter::clean_data_string(deposite_amount);
                let amount_value = cleaned.parse::<f64>().unwrap_or(0.0);
                if amount_value != 0.0 {
                    row_data.amount = Amount::new_deposit(amount_value);
                }
            });

            range.get((row, 7)).inspect(|&balance| {
                // println!("balance: {:?}", DataConverter::clean_data_string(balance))
                let cleaned = DataConverter::clean_data_string(balance);
                row_data.balance = cleaned.parse::<f64>().unwrap_or(0.0);
            });

            row += 1;
            db.add_record(row_data);
        }

        return Result::Ok(db);
    }

    /// Get the transaction data table row numbers. The end number is inclusive.
    fn get_table_range(&self, range: &Range<Data>) -> (usize, usize) {
        let mut start_row = 0;
        let mut end_row = 0;
        let mut row = 12;
        while row < range.end().unwrap().0 as usize {
            match range.get((row, 0)) {
                None => (),
                Some(value) => {
                    // println!("row: {} value: {}", row, value);
                    match DataConverter::get_int(value) {
                        None => (),
                        Some(value) => {
                            // println!("Got int value: {}", value);
                            if value == 1 {
                                start_row = row;
                                break;
                            }
                        }
                    }
                }
            }

            row += 1;
        }

        while row < range.end().unwrap().0 as usize {
            match range.get((row, 0)) {
                None => (),
                Some(value) => {
                    // println!("row: {} value: {}", row, value);
                    match DataConverter::get_int(value) {
                        None => {
                            // println!("Got string value");
                            end_row = row - 1;
                            break;
                        }
                        Some(_) => (),
                    }
                }
            }

            row += 1;
        }

        return (start_row, end_row);
    }
}

pub struct EntryBuilder {}
