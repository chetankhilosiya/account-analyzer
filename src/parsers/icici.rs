use anyhow::Result;
use std::{path::Path, str::FromStr};

use calamine::{Data, DataType, Range, Reader, Xls, open_workbook};

use crate::{db, parsers::DataConverter};

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
            // let row_data = db::Entry::new();
            range.get((row, 1)).inspect(|&date| {
                println!("value date: {:?}", DataConverter::clean_data_string(date))
            });

            range.get((row, 2)).inspect(|&date| {
                println!(
                    "transaction date: {:?}",
                    DataConverter::clean_data_string(date)
                )
            });

            range.get((row, 4)).inspect(|&detail| {
                println!(
                    "transaction details: {:?}",
                    DataConverter::clean_data_string(detail)
                )
            });

            range.get((row, 5)).inspect(|&withdrawal| {
                println!(
                    "withdrawal: {:?}",
                    DataConverter::clean_data_string(withdrawal)
                )
            });
            range.get((row, 6)).inspect(|&deposite| {
                println!("deposite: {:?}", DataConverter::clean_data_string(deposite))
            });
            range.get((row, 7)).inspect(|&balance| {
                println!("balance: {:?}", DataConverter::clean_data_string(balance))
            });

            row += 1;
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
