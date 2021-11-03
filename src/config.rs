use crate::weather::W;
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    city: String,
}

pub fn parse_argv() -> Result<Config, Box<dyn std::error::Error>> {
    let matches = App::new("rust-weather")
        .version("1.0")
        .author("MrBanana. <tomsawyer404@outlook.com>")
        .about("Tell you the city weather")
        .arg(
            Arg::with_name("cityname")
                .value_name("CITY NAME")
                .help("Sets the city name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    Ok(Config {
        city: matches.value_of("cityname").unwrap().to_string(),
    })
}

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    W::get_weather_info(&config.city).await?;
    Ok(())
}
