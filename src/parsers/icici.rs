use anyhow::Result;
use std::{any::Any, fmt::Debug, path::Path};

use calamine::{Data, DataType, Range, Reader, Xls, open_workbook};

use crate::{db::DB, parsers::DataConverter};

pub struct ICICIParser {}

impl ICICIParser {
    pub fn new() -> Self {
        ICICIParser {}
    }

    pub fn parse_file(&self, file: &Path) -> Result<DB> {
        let mut workbook: Xls<_> = open_workbook(file).expect("Can't open file");
        let mut db = DB::new(crate::Bank::ICICI);
        let sheets = workbook.worksheets();
        assert!(sheets.len() == 1, "Sheets are empty");
        let sheet = sheets.first().expect("First sheet should be present");
        let range = &sheet.1;
        if range.is_empty() {
            return Result::Ok(db);
        }

        let table_range = ICICIParser::get_table_range(&self, range);
        print!("table range: ({}, {})", table_range.0, table_range.1);

        // let start = range
        //     .start()
        //     .expect("Start position should be present for non empty range.");
        // let end = range
        //     .end()
        //     .expect("End position should be present for non empty range.");

        // let mut row: usize = 12;
        // while row < end.0 as usize {
        //     let mut col: usize = 0;
        //     while col < end.1 as usize {
        //         match range.get((row, col)) {
        //             None => (),
        //             Some(value) => println!("Cell ({}, {}): {}", row, col, value),
        //         }
        //         col += 1;
        //     }
        //     row += 1;
        // }

        return Result::Ok(db);
    }

    fn get_table_range(&self, range: &Range<Data>) -> (usize, usize) {
        let mut start_row = 0;
        let mut end_row = 0;
        let mut row = 12;
        while row < range.end().unwrap().0 as usize {
            match range.get((row, 0)) {
                None => (),
                Some(value) => {
                    println!("row: {} value: {}, int?: {}", row, value, value.is_string());
                    if DataConverter::get_int(value) == 1 {
                        println!("The start condition matched at row: {}", row);
                        start_row = row;
                        break;
                    }
                }
            }

            row += 1;
        }

        while row < range.end().unwrap().0 as usize {
            match range.get((row, 0)) {
                None => (),
                Some(value) => {
                    println!("row: {} value: {}", row, value);
                    if !value.is_int() && row != 0 {
                        println!("End condition matched at row : {}", row);
                        end_row = row - 1;
                        break;
                    }
                }
            }

            row += 1;
        }

        return (start_row, end_row);
    }
}
