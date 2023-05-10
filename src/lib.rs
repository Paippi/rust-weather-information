pub mod weather {
    pub async fn get_current_weather(weather_api_key: &String, location: &String) -> Option<f64> {
        let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={location}&appid={weather_api_key}&units=metric "
    );
        let resp = reqwest::get(url).await.unwrap();
        let json: serde_json::Value = resp.json().await.unwrap();
        json["main"]["temp"].as_f64()
    }
}
