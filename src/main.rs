use serde::{Deserialize, Serialize};

const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=46.2022&longitude=6.1457&current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,rain,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&models=icon_seamless";

#[derive(Serialize, Deserialize, Debug)]
struct Current {
    time: String,
    #[serde(rename = "temperature_2m")]
    temperature: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Units {
    time: String,
    #[serde(rename = "temperature_2m")]
    temperature: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Conditions {
    latitude: f32,
    longitude: f32,
    current_units: Units,
    current: Current,
}

#[derive(Debug)]
struct CurrentConditions {
    time: String,
    temperature: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?.json::<Conditions>().await?;
    let cc = CurrentConditions {
        time: resp.current.time.to_string(),
        temperature: resp.current.temperature.to_string(),
    };
    println!("Time: {:?}, Temperature: {:?}", cc.time, cc.temperature);
    Ok(())
}
