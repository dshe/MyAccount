
pub enum AccountType {
    Revenue,
    Expense,
    Asset,
    Liability,
    Capital,
    IncomeTax
}

pub struct Account {
    pub account_id:   u32,
    pub account_type: AccountType,
    pub name:         String
}

pub struct Journal {
    pub journal_id: u32,
    pub title:      String
}

pub struct Transaction {
    pub transaction_id: u32,
    pub date:           chrono::DateTime<chrono::Utc>,
    pub journal_id:     u32,
    pub account_id:     u32,
    pub amount:         rust_decimal::Decimal, // credit/debit = positive/negative
    pub description:    String
}

impl Transaction {
    pub fn to_string(self) -> String {
        format!("transaction_id={}, date={}, journal_id={}, account_id={}, amount={:+} description='{}'",
            self.transaction_id,
            self.date,
            self.journal_id,
            self.account_id,
            self.amount,
            self.description)
    }
}