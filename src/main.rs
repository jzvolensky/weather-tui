use dotenv::dotenv;

mod weather;
mod models;
mod user;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let city = user::user_input();
    match weather::Weather::fetch_weather(&city).await {
        Ok(weather) => println!("{}", weather.get_weather()),
        Err(e) => println!("Error: {}", e),
    }
}

