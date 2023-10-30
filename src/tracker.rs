use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::str::FromStr;
use chrono::{Datelike, Timelike, Utc};
use std::io;

pub struct Entry {
    pub id: u32,
    pub username: String,
    pub description: String,
    pub amount: f64,
    pub hour: u32,
    pub minute: u32,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub entry_type: String,
}

pub struct Tracker {
    pub last_entry_id: u32,
}

impl Tracker {

    pub fn add_entry(
        &mut self,
        username: &str,
        description: String,
        amount_str: &str,
        entry_type: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.last_entry_id += 1;
        let id = self.last_entry_id; 

        let current_date = Utc::now();
        let hour = (current_date.hour() + 7) % 24;
        let minute = current_date.minute();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();

        let amount: f64 = parse_float(amount_str)?;

        let entry = Entry {
            id,
            username: username.to_string(),
            description,
            amount,
            hour,
            minute,
            year,
            month,
            day,
            entry_type: entry_type.to_string(),
        };

        self.append_entry(&entry)?;

        Ok(())
    }

    pub fn view_money(&self, username: &str) -> Result<(), Box<dyn Error>> {
        let entries = self.load_entries()?;
        let user_entries: Vec<&Entry> = entries
            .iter()
            .filter(|entry| entry.username == username)
            .collect();

        if !user_entries.is_empty() {
            println!("Income and Expenses for user '{}':", username);
            for entry in &user_entries {
                if entry.entry_type == "Expense" {
                    println!("Expense: {}", entry.description);
                    println!("Amount: ${}", entry.amount);
                    println!("Time: {}:{}",entry.hour, entry.minute);
                    println!("Date of the income: {}-{}-{}",entry.year, entry.month, entry.day);
                } else if entry.entry_type == "Income" {
                    println!("Expense: {}", entry.description);
                    println!("Amount: ${}", entry.amount);
                    println!("Time: {}:{}",entry.hour, entry.minute);
                    println!("Date of the income: {}-{}-{}",entry.year, entry.month, entry.day);
                }
            }

            let total_expenses: f64 = user_entries
                .iter()
                .filter(|entry| entry.entry_type == "Expense")
                .map(|entry| entry.amount)
                .sum();
            let total_incomes: f64 = user_entries
                .iter()
                .filter(|entry| entry.entry_type == "Income")
                .map(|entry| entry.amount)
                .sum();
            println!("Total expenses: ${}", total_expenses);
            println!("Total incomes: ${}",total_incomes);
            println!("Balance: ${}", total_incomes - total_expenses);
            Ok(())
        } else {
            println!("User '{}' not found.", username);
            Err("User not found.".into())
        }
    }

    pub fn delete_user_data(&self, username: &str) -> Result<(), Box<dyn Error>> {
        let mut entries = self.load_entries()?;
        entries.retain(|entry| entry.username != username);
        self.save_all_entries(&entries)?;
        println!("User '{}' data has been deleted.", username);
        Ok(())
    }

    fn append_entry(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("expenses.txt")?;

        writeln!(file, "Entry ID: {}", entry.id)?;
        writeln!(file, "Username: {}", entry.username)?;
        writeln!(file, "Entry Type: {}", entry.entry_type)?;
        writeln!(file, "Description: {}", entry.description)?;
        writeln!(file, "Amount: ${}", entry.amount)?;
        writeln!(file, "Time: {}:{}", entry.hour, entry.minute)?;
        writeln!(file, "Date: {}-{}-{}", entry.year, entry.month, entry.day)?;
        writeln!(file)?;

        Ok(())
    }

    fn save_all_entries(&self, entries: &[Entry]) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("expenses.txt")?;

        for entry in entries {
            writeln!(file, "Entry ID: {}", entry.id)?;
            writeln!(file, "Username: {}", entry.username)?;
            writeln!(file, "Entry Type: {}", entry.entry_type)?;
            writeln!(file, "Description: {}", entry.description)?;
            writeln!(file, "Amount: ${}", entry.amount)?;
            writeln!(file, "Time: {}:{}", entry.hour, entry.minute)?;
            writeln!(file, "Date: {}-{}-{}", entry.year, entry.month, entry.day)?;
            writeln!(file)?;
        }

        Ok(())
    }

    fn load_entries(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("expenses.txt")?;

        let mut reader = io::BufReader::new(&file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let entries: Vec<Entry> = content
            .split("\n\n")
            .filter(|entry_str| !entry_str.is_empty())
            .map(|entry_str| {
                let mut lines = entry_str.lines();
                let id = u32::from_str(lines.next().unwrap().split(": ").nth(1).unwrap()).unwrap();
                let username = lines.next().unwrap().split(": ").nth(1).unwrap().to_string();
                let entry_type = lines.next().unwrap().split(": ").nth(1).unwrap().to_string();
                let description = lines.next().unwrap().split(": ").nth(1).unwrap().to_string();
                let amount = f64::from_str(lines.next().unwrap().split(": $").nth(1).unwrap()).unwrap();
                let time_parts: Vec<&str> = lines.next().unwrap().split(":").collect();
                let hour = u32::from_str(time_parts[1].trim()).unwrap();
                let minute = u32::from_str(time_parts[2].trim()).unwrap();
                let date_parts: Vec<&str> = lines.next().unwrap().split("-").collect();
                let year = i32::from_str(date_parts[0].split(": ").nth(1).unwrap()).unwrap();
                let month = u32::from_str(date_parts[1]).unwrap();
                let day = u32::from_str(date_parts[2]).unwrap();

                Entry {
                    id,
                    username,
                    description,
                    amount,
                    hour,
                    minute,
                    year,
                    month,
                    day,
                    entry_type,
                }
            })
            .collect();

        Ok(entries)
    }
}

fn parse_float(input: &str) -> Result<f64, Box<dyn Error>> {
    f64::from_str(input).map_err(|e| e.into())
}

