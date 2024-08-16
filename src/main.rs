use std::fmt::Display;

const BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

const LATITUDE: f32 = 46.2022;
const LONGITUDE: f32 = 6.1457;
const PARAMETERS: [&str; 8] = [
    "temperature_2m",
    "relative_humidity_2m",
    "precipitation",
    "rain",
    "pressure_msl",
    "surface_pressure",
    "wind_speed_10m",
    "wind_direction_10m",
];
const MODEL: &str = "icon_seamless";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(render_url(BASE_URL, LATITUDE, LONGITUDE))
        .await
        .expect("API requeste failed")
        .json::<serde_json::Value>()
        .await
        .expect("Response parsing failed");
    if let (Some(values), Some(units)) = (
        resp.get("current").expect("Invalid response").as_object(),
        resp.get("current_units")
            .expect("Invalid response")
            .as_object(),
    ) {
        for value in values {
            println!(
                "{}: {}{}",
                value.0,
                value.1,
                units[value.0].as_str().unwrap()
            );
        }
    }
    Ok(())
}

fn render_url(base_url: &str, latitude: f32, longitude: f32) -> String {
    let mut url: String = base_url.to_string() + "?";
    url.push_str(&render("latitude", latitude));
    url.push('&');
    url.push_str(&render("longitude", longitude));
    url.push_str("&current=");
    for parameter in PARAMETERS {
        url.push_str(parameter);
        url.push(',');
    }
    url.pop();
    url.push('&');
    url.push_str(&render("models", MODEL));
    url
}

fn render<T: Display>(parameter: &str, value: T) -> String {
    format!("{}={}", parameter, value)
}
