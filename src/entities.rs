pub enum AccountType {
    Revenue,
    Expense,
    Asset,
    Liability,
    Capital,
    IncomeTax
}

pub struct Account {
    pub id: u32,
    pub account_type: AccountType,
    pub name: String
}

pub struct Journal {
    pub id: u32,
    pub name: String
}

pub struct Transaction {
    pub id: u32,
    pub journal: u32,
    pub account: u32,
    pub amount: rust_decimal::Decimal, // credit/debit: positive/negative
    pub description: String
}

impl Transaction {
    pub fn to_string(self) -> String {
        format!("id={}, journal={}, account={}, amount={:+} description='{}'",
            self.id, 
            self.journal, 
            self.account, 
            self.amount, 
            self.description)
    }
}