use std::io;
use chrono::{DateTime, Utc, TimeZone, NaiveDate, Local};
fn print_menu() {
    println!("Please select number 1-4:");
    println!("1. View all items");
    println!("2. Add item");
    println!("3. Edit item");
    println!("4. Clear all items");
    println!("5. Quit");
    println!();
}

enum Action {
    View,
    Add,
    Edit,
    Quit,
    Clear,
    Invalid,
}

fn action(choice: i32) -> Action {
    match choice {
        1 => Action::View,
        2 => Action::Add,
        3 => Action::Edit,
        4 => Action::Clear,
        5 => Action::Quit,
        _ => Action::Invalid,
    }
}

fn conduct_action(action: Action, todo_list: &mut Vec<String>) -> bool {
    match action {
        Action::View => {
            if todo_list.is_empty() {
                println!("No todos found");
            } else {
                for (i, item) in todo_list.iter().enumerate() {
                    println!("{}: {}", i, item);
                }
            }
            true
        }
        Action::Add => {
            println!();
            let date= read_date();
            

            println!("Enter a new todo: ");
            let item = read_line_trimmed();
            if !item.is_empty() {
                todo_list.push(item);
                println!("New item added");
            }
            true
        }
        Action::Edit => {
            if todo_list.is_empty() {
                println!("No todos to edit");
                return true;
            }
            println!();
            println!("What item would you like to edit?");
            let index = get_input() as usize;

            if index == 0 ||  index > todo_list.len() {
                println!("Invalid item number");
                return true;
            }
            println!();
            println!("Enter new text: ");
            let text = read_line_trimmed();
            todo_list[index - 1] = text;
            println!("Item Updated");
            true
        }
        Action::Clear => {
            println!();
            println!("Cleared all items");
            todo_list.clear();
            true
        }
        Action::Quit => {
            println!();
            println!("Goodbye!");
            false
        }
        Action::Invalid => {
            println!();
            println!("Invalid action");
            true
        }
    }
}

fn read_date() -> NaiveDate {
    let format = "%Y-%m-%d";

    loop {
        println!("Enter todo date (YYYY-MM-DD):");

        let mut date_str = String::new();
        io::stdin()
            .read_line(&mut date_str)
            .expect("Failed to read line");

        let date_str = date_str.trim();

        match NaiveDate::parse_from_str(date_str, format) {
            Ok(date) => return date,
            Err(_) => println!("Invalid date. Try again (example: 2026-02-18)."),
        }
    }
}

fn read_line_trimmed() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn get_input() -> i32 {
    read_line_trimmed().parse::<i32>().unwrap_or(-1)
}

fn main() {
    println!("****** TODO APP ******");
    let mut todo_list: Vec<String> = Vec::new();

    loop {
        print_menu();
        let choice = get_input();
        let action = action(choice);
        let keep_running = conduct_action(action, &mut todo_list);
        if !keep_running {
            break;
        }
        println!();
    }
}