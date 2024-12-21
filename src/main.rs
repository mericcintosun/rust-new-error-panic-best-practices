// Account trait with error handling
trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    // Deposit method implementation
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Deposit amount must be positive.".to_string())
        } else {
            self.balance += amount;
            println!(
                "{} (Account No: {}) deposited {} TL.",
                self.holder_name, self.account_number, amount
            );
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Withdrawal amount must be positive.".to_string())
        } else if amount > self.balance {
            Err(format!(
                "Insufficient funds: Available balance is {} TL.",
                self.balance
            ))
        } else {
            self.balance -= amount;
            println!(
                "{} (Account No: {}) withdrew {} TL.",
                self.holder_name, self.account_number, amount
            );
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("Ahmet Yılmaz"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 1002,
        holder_name: String::from("Ayşe Demir"),
        balance: 1000.0,
    };

    match account1.deposit(200.0) {
        Ok(_) => println!("Deposit successful."),
        Err(e) => println!("Deposit failed: {}", e),
    }

    match account2.withdraw(1500.0) {
        Ok(_) => println!("Withdrawal successful."),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    match account2.withdraw(300.0) {
        Ok(_) => println!("Withdrawal successful."),
        Err(e) => println!("Withdrawal failed: {}", e),
    }

    println!(
        "{} (Account No: {}) Balance: {} TL",
        account1.holder_name,
        account1.account_number,
        account1.balance()
    );
    println!(
        "{} (Account No: {}) Balance: {} TL",
        account2.holder_name,
        account2.account_number,
        account2.balance()
    );
}
