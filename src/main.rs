use std::io::{self, BufRead};
use rand::Rng;

fn main() {
    let mut running = true;
    while running {
        println!("What would you like to do?");
        println!("[Q]uiz? or [M]agic eight ball? or [C]alculator or [E]nd?");
        let input = scan();
        if input == "Q" {
            quiz();
        } else if input == "M" {
            println!("Your question:");
            let question = &scan();
            println!("{}", magic_eight_ball(question));
        } else if input == "C" {
            calculator();
        } else if input == "E" {
            running = false;
        }
        
    }
}

fn calculator() {
    println!("Enter the first number:");
    let number_1: Result<i32, _> = scan().parse();
    println!("Enter the second number:");
    let number_2: Result<i32, _> = scan().parse();
    println!("Enter the arithmetical operator");
    let input = scan();
    if input == "+" {
        let result = match (number_1, number_2) {
            (Ok(num1), Ok(num2)) => Ok(num1 + num2),
            (Err(err1), Err(_err2)) => Err(err1), // Handle the error case if both numbers are Err
            (Err(err), _) | (_, Err(err)) => Err(err), // Handle the error case if either number is Err
        };
    
        match result {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Error: {}", err),
        };
    } else if input == "-" {
        let result = match (number_1, number_2) {
            (Ok(num1), Ok(num2)) => Ok(num1 - num2),
            (Err(err1), Err(_err2)) => Err(err1), // Handle the error case if both numbers are Err
            (Err(err), _) | (_, Err(err)) => Err(err), // Handle the error case if either number is Err
        };
    
        match result {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Error: {}", err),
        };
    } else if input == "/" {
        let result = match (number_1, number_2) {
            (Ok(num1), Ok(num2)) => Ok(num1 / num2),
            (Err(err1), Err(_err2)) => Err(err1), // Handle the error case if both numbers are Err
            (Err(err), _) | (_, Err(err)) => Err(err), // Handle the error case if either number is Err
        };
    
        match result {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Error: {}", err),
        };
    } else if input == "*" {
        let result = match (number_1, number_2) {
            (Ok(num1), Ok(num2)) => Ok(num1 * num2),
            (Err(err1), Err(_err2)) => Err(err1), // Handle the error case if both numbers are Err
            (Err(err), _) | (_, Err(err)) => Err(err), // Handle the error case if either number is Err
        };
    
        match result {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Error: {}", err),
        };
    }
}

fn quiz() {
    let mut rng = rand::thread_rng();

    let random_int = rng.gen_range(-5..=0);
    let questions: [&str; 3] = ["Are turtles reptiles?", "Is rust an awesome language?", "UwU?"];
    let mut score: i32 = 0;
    
    println!("{}", questions[0]);
    if check_input("Yes") == true {
        println!("Correct!");
        score += 1;
    } else {
        println!("Wrong!");
    }

    println!("{}", questions[1]);
    if check_input("Yes") == true {
        println!("Correct!");
        score += 1;
    } else {
        println!("Wrong!");
    }
    
    println!("{}", questions[2]);
    if check_input("UwU!") == true {
        println!("Correct!");
        score += 1;
    } else {
        println!("Wrong!");
    }

    println!("{}", score);
    println!("Acshually you're score is: {}", random_int)
}

fn magic_eight_ball(question: &str) -> &str {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(0..=count_letters(question));
    if random_int % 2 == 0 {
        return "Yes";
    } else {
        return "No";
    }
}

fn check_input(solution: &str) -> bool {
    if scan() == solution {
        return true;
    } else {
        // Display the user's input
        return false;
    }
}

fn scan() -> String {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed_input = input.trim().to_owned();
    return trimmed_input;
}

fn count_letters(text: &str) -> usize {
    text.chars()
        .filter(|c| c.is_alphabetic())
        .count()
}