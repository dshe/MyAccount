mod entities;
use chrono::{Utc, TimeZone};

fn main() {

    // create a list of accounts
    let mut accounts: Vec<entities::Account> = Vec::new();

    // create a sales account and add it to the list of accounts
    let account1 = entities::Account {
        id:           1,
        name:         "sales".to_string(),
        account_type: entities::AccountType::Revenue
    };
    accounts.push(account1);

    // create a cash account and add it to the list of accounts
    let account2 = entities::Account {
        id:           2,
        name:         "cash".to_string(),
        account_type: entities::AccountType::Asset
    };
    accounts.push(account2);

    // create a list of journals
    let mut journals: Vec<entities::Journal> = Vec::new();

    // create a journal and add it to the list of journals
    let journal = entities::Journal {
        id:   3,
        name: "sale".to_string()
    };
    journals.push(journal);

    // create a list of transactions
    let mut transactions: Vec<entities::Transaction> = Vec::new();

    // add a transaction to the list of transactions
    let t = entities::Transaction {
        id:          4,
        date:        chrono::Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        description: "processing sale".to_string(),
        journal:     3,
        account:     1,
        amount:      rust_decimal::Decimal::new(200, 1)
    };
    transactions.push(t);

    // add another transaction to the list of transactions
    let t = entities::Transaction {
        id:          5,
        date:        Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        description: "collecting cash".to_string(),
        journal:     3,
        account:     2,
        amount:      rust_decimal::Decimal::new(-200, 1)
    };
    transactions.push(t);

    // print all transactions
    for tt in transactions {
        println!("Transaction: {0}", tt.to_string());
    }
}
