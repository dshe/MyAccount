use rand::random;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

pub enum AccountType {
    Revenue,
    Expense,
    Asset,
    Liability,
    Equity
}


pub struct Account {
    pub account_id:   u64,
    pub account_type: AccountType,
    pub name:         String
}

impl Account {
    pub fn new(name: &str, account_type: AccountType) -> Account {
        Account {
            account_id: random::<u64>(),
            account_type: account_type,
            name: name.to_string()
        }
    }
}


// a Journal Entry represents a group of related transactions that balance (sum to zero).
pub struct Journal {
    pub journal_id: u64,
    pub title:      String
}

impl Journal {
    pub fn new(title: &str) -> Journal {
        Journal {
            journal_id: random::<u64>(),
            title: title.to_string()
        }
    }
}


pub struct Transaction {
    pub date:           DateTime<Utc>,
    pub journal_id:     u64,
    pub account_id:     u64,
    pub amount:         Decimal, // credit/debit = positive/negative
    pub description:    String
}

impl Transaction {
    pub fn to_string(&self) -> String {
        format!("date={}, journal_id={}, account_id={}, amount={:+} description='{}'",
            self.date,
            self.journal_id,
            self.account_id,
            self.amount,
            self.description)
    }
}
