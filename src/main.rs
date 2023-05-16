use std::io::{self, BufRead};


fn main() {
    let questions: [&str; 3];
    questions = ["Are turtles reptiles?", "Is Rust an awesome language?", "UwU?"];
    let mut score = 0;
    
    println!("{}", questions[0]);
    if checkInput("Yes") == true {
        score += 1;
    }
    
    
    println!("{}", questions[1]);
    if checkInput("Yes") == true {
        score += 1;
    }
    
    
    println!("{}", questions[2]);
    if checkInput("UwU!") == true {
        score += 1;
    }

    println!("{}", score);
}

fn checkInput(solution: &str) -> bool {
    #![allow(warnings)]
    let input = scan().trim().to_owned();

    if input == "Yes" {
        println!("Correct!");
        return true;
    } else {
        // Display the user's input
        println!("Wrong!");
        return false;
    }

}

fn scan() -> String {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}