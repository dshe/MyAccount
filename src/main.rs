mod entities;

fn main() {

    // create the list of accounts
    let mut accounts: Vec<entities::Account> = Vec::new();

    let account1 = entities::Account {
        id: 1,
        name: "drink sales",
        account_type: entities::AccountType::Revenue
    };
    accounts.push(account1);

    let account2 = entities::Account {
        id: 2,
        name: "cash",
        account_type: entities::AccountType::Asset
    };
    accounts.push(account2);

    // create the list of journals and associated transactions
    let mut journals: Vec<entities::Journal> = Vec::new();
    let mut transactions: Vec<entities::Transaction> = Vec::new();

    // add a journal
    let journal = entities::Journal {
        id: 3,
        name: "sales meeting"
    };
    journals.push(journal);

    // add 2 transactions for the journal
    let t = entities::Transaction {
        id: 4,
        name: "wine",
        journal: 3,
        account: 1,
        amount: rust_decimal::Decimal::new(200, 1)
    };
    transactions.push(t);

    let t = entities::Transaction {
        id: 5,
        name: "",
        journal: 3,
        account: 2,
        amount: rust_decimal::Decimal::new(-200, 1)
    };
    transactions.push(t);
}
