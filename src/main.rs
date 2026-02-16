use std::io;
fn print_menu() {
    println!("Please select number 1-4:");
    println!("1. View all items");
    println!("2. Add item");
    println!("3. Edit item");
    println!("4. Quit");
    println!("5. Clear all items");
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
        4 => Action::Quit,
        5 => Action::Clear,
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
            println!("Enter a new item: ");
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
            println!("What item would you like to edit?");
            let index = get_input() as usize;

            if index == 0 ||  index > todo_list.len() {
                println!("Invalid item number");
                return true;
            }

            println!("Enter new text: ");
            let text = read_line_trimmed();
            todo_list[index - 1] = text;
            println!("Item Updated");
            true
        }
        Action::Clear => {
            todo_list.clear();
            true
        }
        Action::Quit => {
            println!("Goodbye!");
            false
        }
        Action::Invalid => {
            println!("Invalid action");
            true
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