use rust_decimal::Decimal;
use chrono::{Utc, TimeZone};

mod entities;

fn main() {

    // create a list of accounts
    let mut accounts: Vec<&entities::Account> = Vec::new();

    // create a sales account and add it to the list of accounts
    let account1 = entities::Account {
        account_id:   1,
        name:         "sales account".to_string(),
        account_type: entities::AccountType::Revenue
    };
    accounts.push(&account1);

    // create a cash account and add it to the list of accounts
    let account2 = entities::Account {
        account_id:   2,
        name:         "accounts receivable".to_string(),
        account_type: entities::AccountType::Asset
    };
    accounts.push(&account2);

    // create a list of journals
    let mut journals: Vec<&entities::Journal> = Vec::new();

    // create a journal and add it to the list of journals
    let journal = entities::Journal {
        journal_id: 1,
        title:      "sale of item to be paid in future".to_string()
    };
    journals.push(&journal);

    // create a list of transactions
    let mut transactions: Vec<&entities::Transaction> = Vec::new();

    // add a transaction to the list of transactions
    let transaction1 = entities::Transaction {
        transaction_id: 1,
        date:           Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        description:    "sale of some item".to_string(),
        journal_id:     1,
        account_id:     1,
        amount:         Decimal::new(200, 1)
    };
    transactions.push(&transaction1);

    // add another transaction to the list of transactions
    let transaction2 = entities::Transaction {
        transaction_id: 2,
        date:           Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        description:    "future payment expected".to_string(),
        journal_id:     1,
        account_id:     2,
        amount:         Decimal::new(-200, 1)
    };
    transactions.push(&transaction2);

    // print all transactions
    transactions.iter().for_each(|t| println!("Transaction: {0}", t.to_string()));

    // ensure that the accounts balance
    let sum: Decimal = transactions.iter().map(|t| t.amount).sum();
    assert_eq!(sum, Decimal::new(0, 0));
}
