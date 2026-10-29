use std::io::{self, stdin};
struct ToDo {
    title: String,
    done: bool,
}

fn print_space() {
    println!("\n\n\n");
}

fn main() {
    let mut todos: Vec<ToDo> = Vec::new();

    loop {
        println!("Enter a command (add, list, done, exit): \n");
        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();

        match command.trim() {
            "list" => {
                print_space();
                println!("To-Do List:");
                for (index, todo) in todos.iter().enumerate() {
                    let status = if todo.done { "[x]" } else { "[ ]" };
                    println!("{} {} {}", index + 1, status, todo.title);
                }
                print_space();
            }
            "add" => {
                println!("Enter the title of the to-do item:");
                let mut title = String::new();
                stdin().read_line(&mut title).unwrap();
                let todo = ToDo {
                    title: title.trim().to_string(),
                    done: false,
                };
                todos.push(todo);
                print_space();
            }
            "done" => {
                println!("Enter the number of the to-do item to mark as done:");
                let mut index_str = String::new();
                stdin().read_line(&mut index_str).unwrap();

                if index_str.is_empty() {
                    println!("Please put a valid number");
                    print_space();
                } else {
                    let index: usize = index_str.trim().parse().unwrap();
                    if index == 0 || index > todos.len() {
                        println!("Invalid index. Please try again.");
                        print_space();
                        continue;
                    }
                    todos[index - 1].done = true;
                    print_space();
                }
            }
            "exit" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid command. Please try again.");
            }
        }
    }
}
