trait Account{
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount{
    _account_number: u64,
    holder_name: String,
    balance: f64
}

impl Account for BankAccount{
    fn deposit(&mut self, amount: f64){
        self.balance += amount;
    }
   fn withdraw(&mut self, amount: f64){
    self.balance -= amount;
   }
    fn balance(&self) -> f64{
        self.balance
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut account1 = BankAccount{_account_number: 1234567, holder_name: String::from("Rasel"), balance: 0.0};
    let mut account2 = BankAccount{_account_number: 1234568, holder_name: String::from("Ronok"), balance: 0.0};
    println!("{:-^46}", " INITIAL BALANCES ");
    println!("{:<16} | {:>15.2}", account1.holder_name, account1.balance());
    println!("{:<16} | {:>15.2}", account2.holder_name, account2.balance());

    println!("\n{:-^46}", " ACCOUNT 1 TRANSACTIONS ");
    account1.deposit(400.0);
    println!("{:<24} {:>10.2}", "After deposit:", account1.balance());
    account1.withdraw(30.0);
    println!("{:<24} {:>10.2}", "After withdrawal:", account1.balance());

    println!("\n{:-^46}", " ACCOUNT 2 TRANSACTIONS ");
    account2.deposit(500.0);
    println!("{:<24} {:>10.2}", "After deposit:", account2.balance());
    account2.withdraw(50.0);
    println!("{:<24} {:>10.2}", "After withdrawal:", account2.balance());
    Ok(())
}