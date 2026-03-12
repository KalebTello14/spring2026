#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        if initial_balance < 0.0 {
            BankAccount { balance: 0.0 }
        } else {
            BankAccount { balance: initial_balance}
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance{
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
    // Bonus method
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!((account.balance() - 150.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert!((account.balance() - 60.0).abs() < EPSILON);
    }

    // Add more tests here
     #[test]
    fn test_deposit_negative() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
     #[test]
    fn test_withdraw_negative() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(-30.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
     #[test]
    fn test_withdraw_too_much() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.10); // 10% interest
        assert!((account.balance() - 110.0).abs() < EPSILON);
    }

    #[test]
    fn test_apply_negative_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(-0.10);
        assert!((account.balance() - 100.0).abs() < EPSILON);
    }
}