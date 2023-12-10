//! Crate to retrieve weather information using rust!
//!
//! # Examples
//!
//! ```bash
//! $ WEATHER_API_KEY=<api key for openweathermap.org> WEATHER_LOCATION=<location> cargo run
//! # e.g.
//! $ WEATHER_API_KEY=1234567890 WEATHER_LOCATION=Helsinki,FI cargo run
//! # or
//! $ WEATHER_API_KEY=1234567890 WEATHER_LOCATION=Helsinki cargo run
//! ```
//!
//! ```rust
//! let weather_api_key = env::var("WEATHER_API_KEY")?;
//! let location = env::var("WEATHER_LOCATION")?;
//! let res = get_current_weather(&weather_api_key, &location)
//!     .await
//!     .ok_or("Location not found...");
//! match res {
//!     Ok(val) => println!("{}", val),
//!     _ => println!("Location not found..."),
//! };
//! Ok(())
//! ```
pub mod weather {
    //! Module for retrieving weather information.

    /// Returns the current weather infromation from [openweathermap API](https://api.openweathermap.org)
    pub async fn get_current_weather(weather_api_key: &String, location: &String) -> Option<f64> {
        let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={location}&appid={weather_api_key}&units=metric "
    );
        let resp = reqwest::get(url).await.unwrap();
        let json: serde_json::Value = resp.json().await.unwrap();
        json["main"]["temp"].as_f64()
    }
}
