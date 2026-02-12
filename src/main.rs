use std::io;
fn print_menu() {
    println!("Please select number 1-4:");
    println!("1. View all items");
    println!("2. Add item");
    println!("3. Edit item");
    println!("4. Quit");
}

fn main()  {
    println!("****** TODO APP ******");
    print_menu();
    let choice = get_input();
    let result = determine_task(choice);

    println!("You selected: {result}");
}

fn get_input() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
        .trim()
        .parse::<i32>()
        .unwrap_or(-1)
}

fn determine_task(input: i32) -> String {
    match input {
        1 => "View all items".to_string(),
        2 => "Add item".to_string(),
        3 => "Edit item".to_string(),
        4 => "Quit".to_string(),
        _ => "Invalid option (pick 1-4)".to_string(),
    }
}