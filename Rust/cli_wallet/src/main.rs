
#[derive(Debug)] 
// enum Currency{
//     SOL,
//     USDC,
// }

struct Wallet {
    address : String,
    balance : f64, 
}

impl Wallet {
    fn new(address : String , starting_balance : f64) -> Self {
        Wallet {
            address, 
            balance : starting_balance, 
        }
    }
    fn send_funds(&mut self, amount: f64, receiver: &str) -> Result<String, String>{
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
    fn print_info(&self) {
        println!("Address: {}",self.address);
        println!("Balance: {}",self.balance);
    }
//  updated add funds
     fn add_funds(&mut self, amount: f64) -> Result<String, String>{
        if amount <= 0.0 {
            return Err("Amount is not valid ...".to_string());
        } 
        else {
            self.balance += amount; 
            Ok(format!("Successfully airdrop {} SOL to Your Account , Balance is {}", amount , self.balance)) 
        }
    }
 
}

use std::io;

fn main() {
    let  mut  my_wallet =  Wallet::new("FfUHUW0uwrghagfiahuBG".to_string(),00.0);

    println!("Welcome to the Rust CLI wallet simulator !");

    loop {
        println!("\n Choose an action : \n\t 1) Check Balance \n\t 2) Send Funds \n\t 3) Add Funds  \n\t 4) Exit ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1"  => {
                my_wallet.print_info();
            }
            "2" => {
                println!("Enter amount to send : ");
                let mut amount_input = String::new();
                io::stdin().read_line(&mut amount_input).expect("Failed to read line");

                let amount: f64 = amount_input.trim().parse().unwrap_or(0.0);

                println!("Enter receivers address : ");
                let  mut receiver = String::new();
                io::stdin().read_line(&mut receiver).expect("Failed to read line");

                match my_wallet.send_funds(amount, receiver.trim()) {
                    Ok(message) => println!("👍 {}",message),
                    Err(e) => println!("👎 {}",e),
                }
            }
// updated to add funds
            "3" => {
                println!("Enter amount to Add Funds : ");
                let mut amount_input_add_funds = String::new();
                io::stdin().read_line(&mut amount_input_add_funds).expect("Failed to read line");

                let amount: f64 = amount_input_add_funds.trim().parse().unwrap_or(0.0);

                match my_wallet.add_funds(amount) {
                    Ok(message) => println!("👍 {}",message),
                    Err(e) => println!("👎 {}",e),
                }
            }

            "4" => {
                println!("Goodbye!!!");
                break;
            }
            _=> println!("Invalid option, Please try again ."),
        }
    }
}