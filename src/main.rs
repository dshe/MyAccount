mod entities;

fn main() {

    // create a list of accounts
    let mut accounts: Vec<entities::Account> = Vec::new();

    // create a sales account and addd it to the list
    let account1 = entities::Account {
        id: 1,
        name: "sales",
        account_type: entities::AccountType::Revenue
    };
    accounts.push(account1);

    // create a cash account and addd it to the list
    let account2 = entities::Account {
        id: 2,
        name: "cash",
        account_type: entities::AccountType::Asset
    };
    accounts.push(account2);

    // create a list of journals
    let mut journals: Vec<entities::Journal> = Vec::new();

    // create a journal and add it to the list
    let journal = entities::Journal {
        id: 3,
        name: "sale"
    };
    journals.push(journal);

    // create a list of transactions
    let mut transactions: Vec<entities::Transaction> = Vec::new();

    // add a transaction to the list for the journal
    let t = entities::Transaction {
        id: 4,
        name: "processing sale",
        journal: 3,
        account: 1,
        amount: rust_decimal::Decimal::new(200, 1)
    };
    transactions.push(t);

    // add another transaction to the list for the journal
    let t = entities::Transaction {
        id: 5,
        name: "collecting cash",
        journal: 3,
        account: 2,
        amount: rust_decimal::Decimal::new(-200, 1)
    };
    transactions.push(t);

    // print all transactions
    for tt in transactions {
        println!("Transaction: {0}", tt.to_string());
    }
}
