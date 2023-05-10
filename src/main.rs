use std::env;
use weather_information::weather::get_current_weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather_api_key = env::var("WEATHER_API_KEY")?;
    let location = env::var("WEATHER_LOCATION")?;
    let res = get_current_weather(&weather_api_key, &location)
        .await
        .ok_or("foo");
    match res {
        Ok(val) => println!("{}", val),
        _ => println!("Nope"),
    };
    Ok(())
}
