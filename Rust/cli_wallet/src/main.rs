use cli_wallet::homepage;
use cli_wallet::wallet;

fn main() {
    let  mut  my_wallet =  wallet::Wallet::new("FfUHUW0uwrghagfiahuBG".to_string(),00.0);

    println!("Welcome to the Rust CLI wallet simulator !");
    homepage::homepage(&mut my_wallet);   
}