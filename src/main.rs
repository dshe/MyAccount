use rust_decimal::Decimal;
use chrono::{Utc, TimeZone};

mod entities;

fn main() {

    // create a list of accounts
    let mut accounts: Vec<&entities::Account> = Vec::new();

    // create a sales account and add it to the list of accounts
    let account1 = entities::Account::new("sales account", entities::AccountType::Revenue);
    accounts.push(&account1);

    // create a cash account and add it to the list of accounts
    let account2 = entities::Account::new("accounts receivable", entities::AccountType::Asset);
    accounts.push(&account2);


    // create a list of journals
    let mut journals: Vec<&entities::Journal> = Vec::new();

    // create a journal and add it to the list of journals
    let journal = entities::Journal::new("sale of item to be paid in future");
    journals.push(&journal);


    // create a list of transactions
    let mut transactions: Vec<&entities::Transaction> = Vec::new();

    // add a transaction to the journal and to the list of transactions
    let transaction1 = entities::Transaction {
        date:           Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        journal_id:     journal.journal_id,
        account_id:     account1.account_id,
        amount:         Decimal::new(200, 1),
        description:    "sale of some item".to_string(),
    };
    transactions.push(&transaction1);

    // add another transaction to the same journal and to the list of transactions
    let transaction2 = entities::Transaction {
        date:           Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        journal_id:     journal.journal_id,
        account_id:     account2.account_id,
        amount:         Decimal::new(-200, 1),
        description:    "future payment expected".to_string(),
    };
    transactions.push(&transaction2);

    // print all transactions
    transactions.iter().for_each(|t| println!("Transaction: {0}", t.to_string()));

    // ensure that the accounts balance
    let sum: Decimal = transactions.iter().map(|t| t.amount).sum();
    assert_eq!(sum, Decimal::new(0, 0));
}
