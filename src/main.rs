use std::io::{self, Write};

fn main() {
    let mut balance = 1000.0; // Starting balance

    loop {
        // Display menu
        println!("\nATM Simulation");
        println!("1. Check Balance");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Exit");

        // Ask for user choice
        print!("\nPlease enter your choice (1-4): ");
        io::stdout().flush().unwrap(); // To ensure the prompt is printed before reading input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 4.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Your balance is: ${:.2}", balance);
            }
            2 => {
                println!("Enter the amount to deposit:");
                let mut deposit = String::new();
                io::stdin().read_line(&mut deposit).unwrap();
                let deposit: f64 = match deposit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount. Please enter a valid number.");
                        continue;
                    }
                };
                balance += deposit;
                println!("You deposited: ${:.2}. New balance: ${:.2}", deposit, balance);
            }
            3 => {
                println!("Enter the amount to withdraw:");
                let mut withdrawal = String::new();
                io::stdin().read_line(&mut withdrawal).unwrap();
                let withdrawal: f64 = match withdrawal.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount. Please enter a valid number.");
                        continue;
                    }
                };
                if withdrawal > balance {
                    println!("Insufficient funds. Your balance is: ${:.2}", balance);
                } else {
                    balance -= withdrawal;
                    println!("You withdrew: ${:.2}. New balance: ${:.2}", withdrawal, balance);
                }
            }
            4 => {
                println!("Exiting ATM simulation. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}
