use dotenv::dotenv;
use std::env::args;

mod weather;
mod models;
mod user;
mod tui;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a city name as an argument.");
        std::process::exit(1);
    }

    let city = &args[1];
    match weather::Weather::fetch_weather(&city).await {
        Ok(weather) => {
            let weather_data = weather.get_weather();
            match tui::draw_weather_terminal(weather_data) {
                Ok(_) => (),
                Err(e) => println!("Error drawing terminal: {}", e),
            }
        },
        Err(e) => println!("Error fetching weather: {}", e),
    }
}

