use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

const APPID: &str = "4845f22236e074cdac59ae174aa580a3";

#[derive(Debug, Serialize, Deserialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    details: Details,
}

#[derive(Debug, Serialize, Deserialize)]
struct Details {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct W {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Main,
    wind: Wind,
}

impl W {
    pub async fn get_weather_info(city: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city, APPID
        );
        let url = Url::parse(&url).unwrap();
        let respon = reqwest::get(url).await?.json::<W>().await?;
        println!("{:#?}", respon);

        Ok(())
    }
}
