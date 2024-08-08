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

fn main() {
    println!("Game on!");
    
    let matrix:Vec<Vec<usize>> = generate_vector_from_input(); 
    println!("{:?}", matrix);
}
