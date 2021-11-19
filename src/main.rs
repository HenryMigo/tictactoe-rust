use std::io;
use std::process;

fn main() {
    loop {
        // Need to setup the original menu
        setup_menu();

        let mut input = String::new();
        let mut trimmed_input = 0;

        // Need to grab user input and convert to an int
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                trimmed_input = input.trim().parse::<i32>().unwrap();
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }

        println!("{:?}", trimmed_input);

        // Handle input
        match trimmed_input {
            1 => println!("Starting new game..."),
            2 => {
                println!("Quitting game...");
                process::exit(1);
            },
            _ => {
                println!("No options entered");
            }
        }
    }
}

/*
    Sets up the menu for the tic-tac-toe
*/
fn setup_menu()
{
    println!("Welcome to Tic-Tac-Toe!");
    println!("1) New Game");
    println!("2) Quit");
    println!("Please enter 1 or 2 and press Enter...");
}