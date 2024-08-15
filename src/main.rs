use serde::{Deserialize, Serialize};

const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=46.2022&longitude=6.1457&current=temperature_2m,relative_humidity_2m,precipitation,rain,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&models=icon_seamless";

#[derive(Serialize, Deserialize, Debug)]
struct Current {
    time: String,
    #[serde(rename = "temperature_2m")]
    temperature: f32,
    #[serde(rename = "relative_humidity_2m")]
    relative_humidity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Units {
    time: String,
    #[serde(rename = "temperature_2m")]
    temperature: String,
    #[serde(rename = "relative_humidity_2m")]
    relative_humidity: String,
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
    relative_humidity: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?.json::<Conditions>().await?;
    let cc = CurrentConditions {
        time: resp.current.time.to_string(),
        temperature: format!(
            "{}{}",
            resp.current.temperature, resp.current_units.temperature
        ),
        relative_humidity: format!(
            "{}{}",
            resp.current.relative_humidity, resp.current_units.relative_humidity
        ),
    };
    println!(
        "Time: {}, Temperature: {}, Relative Humidity: {}",
        cc.time, cc.temperature, cc.relative_humidity
    );
    Ok(())
}
