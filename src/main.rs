use rust_decimal::Decimal;
use chrono::{Utc, TimeZone};

mod entities;

fn main() {

    // create two accounts and add them to a list of accounts
    let sales_account      = entities::Account::new("sales account",       entities::AccountType::Revenue);
    let receivable_account = entities::Account::new("accounts receivable", entities::AccountType::Asset);

    let mut accounts: Vec<&entities::Account> = Vec::new();
    accounts.push(&sales_account);
    accounts.push(&receivable_account);

    // create a journal and add it to a list of journals
    let journal1 = entities::Journal::new("sale of item to be paid in future");
    let mut journals: Vec<&entities::Journal> = Vec::new();
    journals.push(&journal1);


    // create two transactions and add them to a list
    let transaction1 = entities::Transaction {
        date:        Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        journal_id:  journal1.journal_id,
        account_id:  sales_account.account_id,
        amount:      Decimal::new(200, 1),
        description: "sale of some item".to_string(),
    };

    let transaction2 = entities::Transaction {
        date:        Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        journal_id:  journal1.journal_id,
        account_id:  receivable_account.account_id,
        amount:      Decimal::new(-200, 1),
        description: "future payment expected".to_string(),
    };

    let mut transactions: Vec<&entities::Transaction> = Vec::new();
    transactions.push(&transaction1);
    transactions.push(&transaction2);

    
    // print all transactions
    transactions.iter().for_each(|t| println!("Transaction: {0}", t.to_string()));

    // ensure that the accounts balance
    let sum: Decimal = transactions.iter().map(|t| t.amount).sum();
    assert_eq!(sum, Decimal::new(0, 0));
}
