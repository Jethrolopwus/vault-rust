use vault_rust::lib::{
    bank_system::{accounts::Bank, bank::BankAccount as _},
    bank_txns::{balance_check::CheckingAccount, savings::SavingsAccount},
};

fn main() {
    // Create a Savings Account
    let mut savings = SavingsAccount::new(1000.0, 5.0);
    savings.deposit(500.0);
    savings.withdraw(200.0);
    savings.apply_interest();
    println!("Savings balance after interest: ${}", savings.get_balance());

  
    let mut checking = CheckingAccount::new(500.0, 200.0);
    checking.deposit(300.0);
    checking.withdraw(1000.0);

    let mut bank = Bank::new();
    bank.add_account(Box::new(savings)); 
    bank.add_account(Box::new(checking));

    println!("Total balance in the bank: ${}", bank.total_balance());
}
