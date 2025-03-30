use super::bank::BankAccount;

pub struct Bank {
    accounts: Vec<Box<dyn BankAccount>>, 
}

impl Bank {
    pub fn new() -> Self {
        Self { accounts: Vec::new() }
    }

    pub fn add_account(&mut self, account: Box<dyn BankAccount>) {
        self.accounts.push(account);
    }

    pub fn total_balance(&self) -> f64 {
        self.accounts.iter().map(|acc| acc.get_balance()).sum()
    }
}
