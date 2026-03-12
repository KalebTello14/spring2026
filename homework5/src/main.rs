mod bank_account;

use bank_account::BankAccount;

fn main () {
    let mut account = BankAccount::new(100.0);

    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawal: {}", account.balance());

    // Apply 10% interest
    account.apply_interest(0.10);
    println!("After applying 10% interest: {}", account.balance());

}