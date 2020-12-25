pub enum AccountType {
    Revenue,
    Expense,
    Asset,
    Liability,
    Capital,
    IncomeTax
}

pub struct Account {
    pub id: u16,
    pub account_type: AccountType,
    pub name: &'static str
}

pub struct Journal {
    pub id: u16,
    pub name: &'static str
}

pub struct Transaction {
    pub id: u16,
    pub journal: u16,
    pub account: u16,
    pub amount: rust_decimal::Decimal, // credit/debit: positive/negative
    pub name: &'static str
}

impl Transaction {
    pub fn to_string(self) -> String {
        format!("id={}, journal={}, account={}, amount={} name={}",
            self.id, 
            self.journal.to_string(), 
            self.account.to_string(), 
            self.amount.to_string(), 
            self.name.to_string())
    }
}




