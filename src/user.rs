use std::io::stdin;

pub fn user_input() -> String {
    let mut city = String::new();
    println!("Enter a city name: ");
    stdin().read_line(&mut city).expect("Failed to read line");
    city
}
