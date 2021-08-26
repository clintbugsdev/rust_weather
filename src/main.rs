use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
    sea_level: Option<i32>,
    grnd_level: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: Option<f64>,
    id: Option<i32>,
    country: String,
    sunrise: i32,
    sunset: i32,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = Forecast::get(&args.city, &args.country_code).await?;
    let temp_cel = kelvin_to_celcius(response.main.temp);
    println!(
        "our city: {} our country code: {}, Temperature:{:.2}, Humidity: {}%",
        args.city, args.country_code, temp_cel, response.main.humidity
    );
    Ok(())
}

fn kelvin_to_celcius(kel: f64) -> f64 {
    kel - 273.15
}

impl Forecast {
    async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure> {
        let url:String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid=9b5c6a4883984c9aecdd88503ae2516f", city, country_code);
        let url: Url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;
        Ok(resp)
    }
}
//api.openweathermap.org/data/2.5/weather?q={city name},{state code},{country code}&appid={API key}
//9b5c6a4883984c9aecdd88503ae2516f
