use std::collections::BTreeMap;
use std::io;
use emojis;
use chrono::NaiveDateTime;

type TodoMap = BTreeMap<NaiveDateTime, String>;

fn print_menu() {
    let mag = emojis::get_by_shortcode("mag")
        .map(|e| e.as_str())
        .unwrap_or("🔍");

    let plus = emojis::get_by_shortcode("plus")
        .map(|e| e.as_str())
        .unwrap_or("➕");

    let change = emojis::get_by_shortcode("edit")
        .map(|e| e.as_str())
        .unwrap_or("🔄");

    let subtract = emojis::get_by_shortcode("subtract")
        .map(|e| e.as_str())
        .unwrap_or("➖");

    let stop = emojis::get_by_shortcode("stop")
        .map(|e| e.as_str())
        .unwrap_or("🛑");


    println!("Please select number 1-5:");
    println!("1. {} View all items", mag);
    println!("2. {} Add item", plus);
    println!("3. {} Edit item", change);
    println!("4. {} Clear all items", subtract);
    println!("5. {} Quit", stop);
    println!();
}

enum Action {
    View,
    Add,
    Edit,
    Clear,
    Quit,
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

fn conduct_action(action: Action, todo_list: &mut TodoMap) -> bool {
    match action {
        Action::View => {
            if todo_list.is_empty() {
                println!("(No items yet)");
                return true;
            }

            println!("Todo list (sorted by date/time):");
            for (i, (dt, todo)) in todo_list.iter().enumerate() {
                println!(
                    "{}. {}  -  {}",
                    i + 1,
                    dt.format("%Y-%m-%d %H:%M"),
                    todo
                );
            }
            true
        }

        Action::Add => {
            let due = read_date_time();

            println!("Enter a new todo:");
            let item = read_line_trimmed();

            if item.is_empty() {
                println!("Nothing added.");
                return true;
            }

            // Map keys must be unique
            if todo_list.contains_key(&due) {
                println!("That date/time already has a todo. Pick a different time.");
                return true;
            }

            todo_list.insert(due, item);
            println!("New item added.");
            true
        }

        Action::Edit => {
            if todo_list.is_empty() {
                println!("No todos to edit.");
                return true;
            }

            println!("Which item number do you want to edit?");
            for (i, (dt, todo)) in todo_list.iter().enumerate() {
                println!(
                    "{}. {}  -  {}",
                    i + 1,
                    dt.format("%Y-%m-%d %H:%M"),
                    todo
                );
            }

            let index = get_input() as usize;
            if index == 0 || index > todo_list.len() {
                println!("Invalid item number.");
                return true;
            }

            let old_key = *todo_list.keys().nth(index - 1).unwrap();

            println!("Edit date/time? (yes/no)");
            let edit_dt = read_line_trimmed().to_lowercase();

            let mut new_key = old_key; // start with old key
            match edit_dt.as_str() {
                "yes" | "y" => {
                    new_key = read_date_time();
                }
                "no" | "n" => { /* keep old */ }
                _ => {
                    println!("Please type yes or no.");
                    return true;
                }
            }


            println!("Edit todo text? (yes/no)");
            let edit_text = read_line_trimmed().to_lowercase();

            let mut new_text = todo_list.get(&old_key).unwrap().clone(); // start with old text
            match edit_text.as_str() {
                "yes" | "y" => {
                    println!("Enter new text:");
                    let t = read_line_trimmed();
                    if t.is_empty() {
                        println!("Text cannot be empty.");
                        return true;
                    }
                    new_text = t;
                }
                "no" | "n" => { /* keep old */ }
                _ => {
                    println!("Please type yes or no.");
                    return true;
                }
            }

            todo_list.remove(&old_key);
            if todo_list.contains_key(&new_key) && new_key != old_key {
                println!("That date/time is already used. Edit cancelled.");
                todo_list.insert(old_key, new_text);
                return true;
            }

            todo_list.insert(new_key, new_text);
            println!("Item updated.");

            true
        }

        Action::Clear => {
            todo_list.clear();
            println!("Cleared all items.");
            true
        }

        Action::Quit => {
            println!("Goodbye!");
            false
        }

        Action::Invalid => {
            println!("Invalid option (pick 1-5).");
            true
        }
    }
}

fn read_date_time() -> NaiveDateTime {
    let format = "%Y-%m-%d %H:%M";

    loop {
        println!("Enter todo date and time (YYYY-MM-DD HH:MM):");

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();

        match NaiveDateTime::parse_from_str(s, format) {
            Ok(dt) => return dt,
            Err(_) => println!("Invalid. Use YYYY-MM-DD HH:MM (24-hour)."),
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
    let rocket = emojis::get_by_shortcode("rocket")
        .map(|e| e.as_str())
        .unwrap_or("🚀");
    
    println!("{}****** TODO APP ******{}", rocket, rocket );
    let mut todo_list: TodoMap = BTreeMap::new();

    loop {
        print_menu();
        let choice = get_input();
        let act = action(choice);

        let keep_running = conduct_action(act, &mut todo_list);
        if !keep_running {
            break;
        }

        println!();
    }
}
