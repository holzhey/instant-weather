const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=46.2022&longitude=6.1457&current=temperature_2m,relative_humidity_2m,precipitation,rain,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&models=icon_seamless";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(URL).await?.json::<serde_json::Value>().await?;
    if let Some(values) = resp.get("current").unwrap().as_object() {
        if let Some(units) = resp.get("current_units").unwrap().as_object() {
            for v in values {
                println!("{}: {} {}", v.0, v.1, units[v.0].as_str().unwrap());
            }
        }
    }
    Ok(())
}
