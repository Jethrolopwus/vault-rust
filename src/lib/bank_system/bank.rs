pub trait BankAccount {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> bool;
    fn get_balance(&self) -> f64;
}
