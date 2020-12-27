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

pub struct Journal {
    pub journal_id: u64,
    pub title:      String
}

pub struct Transaction {
    pub transaction_id: u64,
    pub journal_id:     u64,
    pub account_id:     u64,
    pub date:           chrono::DateTime<chrono::Utc>,
    pub amount:         rust_decimal::Decimal, // credit/debit = positive/negative
    pub description:    String
}

impl Transaction {
    pub fn to_string(&self) -> String {
        format!("transaction_id={}, journal_id={}, account_id={}, date={}, amount={:+} description='{}'",
            self.transaction_id,
             self.journal_id,
            self.account_id,
            self.date,
            self.amount,
            self.description)
    }
}