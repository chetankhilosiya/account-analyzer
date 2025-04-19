use chrono::{DateTime, Local};

use crate::Bank;

pub struct DB {
    bank: Bank,
    records: Vec<Entry>,
}

struct Entry {
    transaction_date: DateTime<Local>,
    value_date: DateTime<Local>,
    details: String,
    description: String,
    amount: Amount,
    balance: f64,
}

struct Amount {
    value: f64,
    amount_type: AmountType,
}

enum AmountType {
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
}

impl Entry {
    pub fn default() -> Self {
        Entry {
            transaction_date: Local::now(),
            value_date: Local::now(),
            details: String::new(),
            description: String::new(),
            amount: Amount::default(),
            balance: -1.0,
        }
    }

    pub fn new(
        transaction_date: DateTime<Local>,
        value_date: DateTime<Local>,
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
}

impl DB {
    pub fn new(bank: Bank) -> Self {
        DB {
            bank,
            records: Vec::new(),
        }
    }
}
