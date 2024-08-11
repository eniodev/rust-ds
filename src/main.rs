use std::io::{self, Write};

// Bout to go brrr!! (Oh don't even mind me)
 
// 1- Write a Rust program to create an n x n matrix by taking an integer (n) as input from the user.
fn generate_2dvector_from_input(len: String) -> Vec<Vec<usize>> {
    let len: usize = len.trim().parse().expect("Faile to convert string to int");
    let matrix = vec![vec![0; len]; len];
    matrix
}

// 2- Write a Rust program to capitalize the first character of each element of a given string vector. 
// Return the vector.
fn capitalize_the_first_letter(list:Vec<String>)-> Vec<String> {
    let mut capital_arr: Vec<String> = vec![];
    for el in list {
        capital_arr.push(camel_case(el));
    }
    capital_arr
}

// Just an aux function
fn camel_case(string: String) -> String {
    let mut cameled_string = String::new();
    let first_char = string.as_bytes()[0] as char;
    let upper_first_char = first_char.to_ascii_uppercase();
    
    cameled_string.push(upper_first_char);
    
    for ch in string.trim().chars().skip(1) {
        cameled_string.push(ch);   
    }
    cameled_string
} 

fn main() {
    println!("Game on!");
    
    // switch case:
    println!("Choose from the menu:");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read option...");
    let option: usize = line.trim().parse().expect("Failed to parse line...");
    let _ = match option {
         1 => {
            let mut len = String::new();
            print!("Enter the size of the matrix: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut len)
                .expect("Failed to read line.");
            let matrix:Vec<Vec<usize>> = generate_2dvector_from_input(len); 
            println!("{:?}", matrix); 
        },
        2 => {
            let mut str_arr = vec!["enio".to_string(), "carlos".to_string(), "paulo".to_string()];
            str_arr = capitalize_the_first_letter(str_arr);
            print!("{:?}", str_arr);
        }
        _ => {
            print!("Option out of bounds!")
        }
    };
    
    //let mut list: Vec<String> = vec!["-w".to_string(), "60".to_string(), "args".to_string()];
    //capitalize_the_first_letter(list) [unfinished]
    
    //slice_arr_to_string_arr();
}

// Manually copy a slice array into a String array
fn slice_arr_to_string_arr() -> () {
    let string_arr: [&str; 3] = ["hello", "from", "porto"];
    for el in string_arr {
        print!("{} ", el);
    }
    
    let new_string_arr: Vec<String> = string_arr.into_iter().map(|el| el.to_owned()).collect();
    
    for el in new_string_arr.into_iter().enumerate() {
        print!("{:?} ", el)
    }
}

