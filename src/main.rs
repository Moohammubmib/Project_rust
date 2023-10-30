use std::error::Error;
use std::io;

mod tracker;
use tracker::Tracker;

fn main() -> Result<(), Box<dyn Error>> {
    let mut expense_tracker = Tracker { last_entry_id: 0 };

    loop {
        println!("Income And Expense Tracker Menu:");
        println!("1. Add Expense");
        println!("2. Add Income");
        println!("3. View Expenses and Incomes");
        println!("4. Delete User Data");
        println!("5. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Enter your username: ");
                let mut username = String::new();
                io::stdin()
                    .read_line(&mut username)
                    .expect("Failed to read line");
                let username = username.trim();

                println!("Enter expense description: ");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                let description = description.trim().to_string();

                println!("Enter expense amount: ");
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");
                let amount = amount.trim();

                if let Err(err) = expense_tracker.add_entry(username, description, amount, "Expense") {
                    eprintln!("Error adding expense: {}", err);
                } else {
                    println!("Expense added successfully.");
                }
            }
            2 => {
                println!("Enter your username: ");
                let mut username = String::new();
                io::stdin()
                    .read_line(&mut username)
                    .expect("Failed to read line");
                let username = username.trim();

                println!("Enter income description: ");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                let description = description.trim().to_string();

                println!("Enter income amount: ");
                let mut amount = String::new();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");
                let amount = amount.trim();

                if let Err(err) = expense_tracker.add_entry(username, description, amount, "Income") {
                    eprintln!("Error adding income: {}", err);
                } else {
                    println!("Income added successfully.");
                }
            }
            3 => {
                println!("Enter your username: ");
                let mut username = String::new();
                io::stdin()
                    .read_line(&mut username)
                    .expect("Failed to read line");
                let username = username.trim();
                if let Err(_) = expense_tracker.view_money(username) {
                    eprintln!("Error viewing expenses and incomes.");
                }
            }
            4 => {
                println!("Enter the username to delete data: ");
                let mut username = String::new();
                io::stdin()
                    .read_line(&mut username)
                    .expect("Failed to read line");
                let username = username.trim();
                if let Err(_) = expense_tracker.delete_user_data(username) {
                    eprintln!("Error deleting user data.");
                } else {
                    println!("User data deleted successfully.");
                }
            }
            5 => {
                println!("Exiting Expense Tracker. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
    }

    Ok(())
}
