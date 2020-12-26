mod entities;
use chrono::TimeZone;

fn main() {

    // create a list of accounts
    let mut accounts: Vec<entities::Account> = Vec::new();

    // create a sales account and add it to the list of accounts
    let account1 = entities::Account {
        account_id:   1,
        name:         "sales".to_string(),
        account_type: entities::AccountType::Revenue
    };
    accounts.push(account1);

    // create a cash account and add it to the list of accounts
    let account2 = entities::Account {
        account_id:   2,
        name:         "cash".to_string(),
        account_type: entities::AccountType::Asset
    };
    accounts.push(account2);

    // create a list of journals
    let mut journals: Vec<entities::Journal> = Vec::new();

    // create a journal and add it to the list of journals
    let journal = entities::Journal {
        journal_id: 1,
        title:      "sale".to_string()
    };
    journals.push(journal);

    // create a list of transactions
    let mut transactions: Vec<entities::Transaction> = Vec::new();

    // add a transaction to the list of transactions
    let transaction1 = entities::Transaction {
        transaction_id: 1,
        date:           chrono::Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        description:    "processing sale".to_string(),
        journal_id:     1,
        account_id:     1,
        amount:         rust_decimal::Decimal::new(200, 1)
    };
    transactions.push(transaction1);

    // add another transaction to the list of transactions
    let transaction2 = entities::Transaction {
        transaction_id: 2,
        date:           chrono::Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 1, 444),
        description:    "collecting cash".to_string(),
        journal_id:     1,
        account_id:     2,
        amount:         rust_decimal::Decimal::new(-200, 1)
    };
    transactions.push(transaction2);

    // print all transactions
    for tt in transactions {
        println!("Transaction: {0}", tt.to_string());
    }
}
