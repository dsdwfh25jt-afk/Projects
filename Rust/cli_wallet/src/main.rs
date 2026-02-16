// we'll use this to simulate different coins later 
#[derive(Debug)] // this allows us to print the Enum using {:?}
enum Currency{
    SOL,
    USDC,
}

struct Wallet {
    address : String,
    balance : f64, // in real solana , we use u64 lamps , but f64 is easier for a simulator 
}

//             //               //               //               //                //

impl Wallet {
    // Constructor function to create new wallet 
    fn new(address : String , starting_balance : f64) -> Self {
        Wallet {
            address, 
            balance : starting_balance, 
        }
    }

    // Consept  check : why &mut self? 
    // Because we are changing the balance (mutating)
    // If we just used self , the wallet ownership would move into this function 
    // and ne destroyed after the transaction ! 
    fn send_funds(&mut self, amount: f64, receiver: &str) -> Result<String, String>{
        if amount <= 0.0 {
            return Err("Amount is not valid ...".to_string());
        } 
        if self.balance >= amount {
            self.balance -= amount; 
            // In a real app , you'd add logic here to update the receivers balance
            Ok(format!("Successfully sent {} SOL to {}", amount , receiver)) 
        } else {
            Err("Insufficient funds".to_string())
        }
    }
    fn print_info(&self) {
        println!("Address: {}",self.address);
        println!("Balance: {}",self.balance);
    }
}


use std::io;

fn main() {
    // 1 . Initialize a wallet with fake address and 10 SOL 
    let  mut  my_wallet =  Wallet::new("FfUHUW0uwrghagfiahuBG".to_string(),10.0);

    println!("Welcome to the Rust CLI wallet simulator !");

    // 2 . Start the main program loop 
    loop {
        println!("\n Choose an action : \n\t 1) Check Balance \n\t 2) Send Funds  \n\t 3) Exit ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        // Trim the newline character and match the input 
        match choice.trim() {
            "1"  => {
                // We are borrowing the wallet immutably here (&self)
                my_wallet.print_info();
            }
            "2" => {
                println!("Enter amount to send : ");
                let mut amount_input = String::new();
                io::stdin().read_line(&mut amount_input).expect("Failed to read line");

                // parse the string into fload (f64)
                // 'unwrap_or' is a safe way to handle parsing errors without crashing
                let amount: f64 = amount_input.trim().parse().unwrap_or(0.0);

                println!("Enter receivers address : ");
                let  mut receiver = String::new();
                io::stdin().read_line(&mut receiver).expect("Failed to read line");

                // we are borrowing wallet MUTABLY here (&mut self)
                match my_wallet.send_funds(amount, receiver.trim()) {
                    Ok(message) => println!("👍 {}",message),
                    Err(e) => println!("👎 {}",e),
                }
            }
            "3" => {
                println!("Goodbye!!!");
                break; // Exit the loop 
            }
            _=> println!("Invalid option, Please try again ."),
        }
    }
}






