
trait Account {
    fn deposit(&mut self, amount: u64);
    fn withdraw(&mut self, amount: u64);
    fn balance(&self) -> u64;
}

#[derive(Debug)]
struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: u64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) {
        self.balance -= amount;
    }

    fn balance(&self) -> u64 {
        self.balance
    }
}


fn main() {

    let mut my_account = BankAccount {
        account_number: 123456789,
        holder_name: "Omer Eraslan".to_string(),
        balance: 1000,
    };

    let mut second_account = BankAccount {
        account_number: 987654321,
        holder_name: "Canberk Yaşa".to_string(),
        balance: 1000,
    };

    println!("My account :{:?}",my_account);
    my_account.deposit(1000);
    println!("After deposit my account balance: {}", my_account.balance());
    my_account.withdraw(500);
    println!("After withdraw my account balance: {}", my_account.balance());

    println!("------------------------------------");

    println!("Second account :{:?}",second_account);
    second_account.deposit(500);
    println!("After deposit second account balance: {}", second_account.balance());
    second_account.withdraw(250);
    println!("After withdraw second account balance: {}", second_account.balance());

}
