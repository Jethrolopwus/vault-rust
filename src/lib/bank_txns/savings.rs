use crate::lib::bank_system::bank::BankAccount;

pub struct SavingsAccount {
    balance: f64,
    interest_rate: f64,
}

impl SavingsAccount {
    pub fn new(initial_balance: f64, interest_rate: f64) -> Self {
        Self {
            balance: initial_balance,
            interest_rate,
        }
    }

    pub fn apply_interest(&mut self) {
        self.balance += self.balance * (self.interest_rate / 100.0);
    }
}

impl BankAccount for SavingsAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${}, new balance: ${}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew ${}, new balance: ${}", amount, self.balance);
            true
        } else {
            println!("Insufficient funds!");
            false
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}
