use crate::lib::bank_system::bank::BankAccount;

pub struct CheckingAccount {
    balance: f64,
    overdraft_limit: f64,
}

impl CheckingAccount {
    pub fn new(initial_balance: f64, overdraft_limit: f64) -> Self {
        Self {
            balance: initial_balance,
            overdraft_limit,
        }
    }
}

impl BankAccount for CheckingAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${}, new balance: ${}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance - amount >= -self.overdraft_limit {
            self.balance -= amount;
            println!("Withdrew ${}, new balance: ${}", amount, self.balance);
            true
        } else {
            println!("Overdraft limit exceeded!");
            false
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}
