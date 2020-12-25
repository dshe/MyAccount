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
    pub name: &'static str,
    pub account_type: AccountType
}

pub struct Journal {
    pub id: u16,
    pub name: &'static str
}

pub struct Transaction {
    pub id: u16,
    pub name: &'static str,    
    pub journal: u16,
    pub account: u16,
    pub amount: rust_decimal::Decimal // credit/debit: positive/negative
}


