use chrono::NaiveDate;

use crate::Bank;

#[derive(Clone, PartialEq, Debug)]
pub struct DB {
    bank: Bank,
    records: Vec<Entry>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Entry {
    pub transaction_date: NaiveDate,
    pub value_date: NaiveDate,
    pub details: String,
    pub description: String,
    pub amount: Amount,
    pub balance: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Amount {
    value: f64,
    amount_type: AmountType,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AmountType {
    Withdrawal,
    Deposit,
}

impl Amount {
    pub fn default() -> Self {
        Amount {
            value: 0.0,
            amount_type: AmountType::Withdrawal,
        }
    }

    pub fn new(value: f64, amount_type: AmountType) -> Self {
        Amount { value, amount_type }
    }

    pub fn new_withdrawal(value: f64) -> Self {
        Amount {
            value,
            amount_type: AmountType::Withdrawal,
        }
    }

    pub fn new_deposit(value: f64) -> Self {
        Amount {
            value,
            amount_type: AmountType::Deposit,
        }
    }
}

impl Entry {
    pub fn default() -> Self {
        Entry {
            transaction_date: NaiveDate::default(),
            value_date: NaiveDate::default(),
            details: String::new(),
            description: String::new(),
            amount: Amount::default(),
            balance: 0.0,
        }
    }

    pub fn new(
        transaction_date: NaiveDate,
        value_date: NaiveDate,
        details: String,
        description: String,
        amount: Amount,
        balance: f64,
    ) -> Self {
        Entry {
            transaction_date,
            value_date,
            details,
            description,
            amount,
            balance,
        }
    }

    pub fn set_transaction_date(&mut self, date_str: &str, fmt: &str) {
        self.transaction_date = match NaiveDate::parse_from_str(date_str, fmt) {
            Ok(date) => date,
            Err(_) => NaiveDate::default(),
        }
    }

    pub fn set_value_date(&mut self, date_str: &str, fmt: &str) {
        self.value_date = match NaiveDate::parse_from_str(date_str, fmt) {
            Ok(date) => date,
            Err(_) => NaiveDate::default(),
        }
    }
}

impl DB {
    pub fn new(bank: Bank) -> Self {
        DB {
            bank,
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, entry: Entry) {
        self.records.push(entry);
    }
}
