mod currency;
pub struct Wallet {
    pub address : String,
    pub balance : f64, 
}

impl Wallet {
    pub fn new(address : String , starting_balance : f64) -> Self {
        Wallet {
            address, 
            balance : starting_balance, 
        }
    }
    pub fn send_funds(&mut self, amount: f64, receiver: &str) -> Result<String, String>{
        if amount <= 0.0 {
            return Err("Amount is not valid ...".to_string());
        } 
        if self.balance >= amount {
            self.balance -= amount; 
            Ok(format!("Successfully sent {} SOL to {}", amount , receiver)) 
        } else {
            Err("Insufficient funds".to_string())
        }
    }
    pub fn print_info(&self) {
        println!("Address: {}",self.address);
        println!("Balance: {}",self.balance);
    }
//  updated add funds
    pub fn add_funds(&mut self, amount: f64) -> Result<String, String>{
        if amount <= 0.0 {
            return Err("Amount is not valid ...".to_string());
        } 
        else {
            self.balance += amount; 
            Ok(format!("Successfully airdrop {} SOL to Your Account , Balance is {}", amount , self.balance)) 
        }
    }
    // Reduce funds
    pub fn debit_funds(&mut self, amount: f64) -> Result<String, String>{
        if amount <= 0.0 {
            return Err("Amount is not valid ...".to_string());
        } 
        else {
            self.balance -= amount; 
            Ok(format!("{} SOL has been Deducted from Account {}, Balance is {}", amount ,self.address, self.balance)) 
        }
    }
 
}