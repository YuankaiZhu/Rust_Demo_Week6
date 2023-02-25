use std::io::{stdin, stdout, Write};

fn main() {
    println!("Welcome to the Rust Data Processing Program!");

    let mut data: Vec<i32> = Vec::new();

    loop {
        print_menu();

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Failed to read user input");

        let choice = choice.trim();

        match choice {
            "1" => {
                // Add data point to the vector
                print!("Enter the data point: ");
                stdout().flush().unwrap();

                let mut data_point = String::new();
                stdin().read_line(&mut data_point).expect("Failed to read user input");

                let data_point = match data_point.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input! Please enter an integer.");
                        continue;
                    }
                };

                data.push(data_point);
                println!("Data point added successfully!");
            },
            "2" => {
                // Display the current data
                if !data.is_empty() {
                    println!("Current data: {:?}", data);
                } else {
                    println!("No data points added yet!");
                }
            },
            "3" => {
                // Calculate the sum of the data
                if !data.is_empty() {
                    let sum: i32 = data.iter().sum();
                    println!("Sum of the data: {}", sum);
                } else {
                    println!("No data points added yet!");
                }
            },
            "4" => {
                // Calculate the average of the data
                if !data.is_empty() {
                    let average: f32 = data.iter().sum::<i32>() as f32 / data.len() as f32;
                    println!("Average of the data: {}", average);
                } else {
                    println!("No data points added yet!");
                }
            },
            "5" => {
                // Clear the data
                data.clear();
                println!("Data cleared!");
            },
            "6" => {
                // Quit the program
                println!("Thanks for using the Rust Data Processing Program!");
                return;
            },
            _ => {
                println!("Invalid choice!");
            },
        }
    }
}

fn print_menu() {
    println!("Menu:");
    println!("1. Add data point");
    println!("2. Display current data");
    println!("3. Calculate sum of data");
    println!("4. Calculate average of data");
    println!("5. Clear data");
    println!("6. Quit");
    print!("Enter your choice: ");
    stdout().flush().unwrap();
}
