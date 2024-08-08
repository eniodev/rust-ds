use std::io;

// Bout to go brrr!! (Oh don't even mind me)
 
// 0.0 Write a Rust program to create an n x n matrix by taking an integer (n) as input from the user.
fn generate_vector_from_input() -> Vec<Vec<usize>> {
   let mut len = String::new();
    io::stdin()
        .read_line(&mut len)
        .expect("Failed to read line.");
    
    let len: usize = len.trim().parse().expect("Faile to convert string to int");
    
    let matrix = vec![vec![0; len]; len];
    return matrix;
}

// 0.1 Write a Rust program to capitalize the first character of each element of a given string vector. 
// Return the vector.

fn capitalize_the_first_letter(list:Vec<String>)-> () {
    for el in list {
        println!("{}", el);
    }
    
}

fn main() {
    println!("Game on!");
    
    // let matrix:Vec<Vec<usize>> = generate_vector_from_input(); 
    // println!("{:?}", matrix);
    
    let mut list: Vec<String> = vec!["-w".to_string(), "60".to_string(), "args".to_string()];
    capitalize_the_first_letter(list)
}
