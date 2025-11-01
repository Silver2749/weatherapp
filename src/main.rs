use serde::Deserialize;
slint::include_modules!();

#[derive(Deserialize, Debug)] //function to deserialize the json and shift it to smaller modules like weather wind etc
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]  //function to take the weather properties and include them in a single description
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)] // f64 should be default any lower and causes an error
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
    temp_min: f64,
    temp_max: f64
}

#[derive(Deserialize, Debug)] //had to make a different struct for wind because integration with slint is causes problems
struct Wind {
    speed: f64,
}

fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );   // where the actual weather info comes from (openweather api)
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn format_weather_info(response: &WeatherResponse) -> String {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;
    let temp_min =response.main.temp_min;
    let temp_max=response.main.temp_max;

    format!(
        "🌍 {}\nCondition: {}\nTemperature: {:.1}°C {}\nHumidity: {:.1}%\nPressure: {:.1} hPa\nWind Speed: {:.1} m/s\nMin Temperature: {:.1}°C\nMax Temperature {:.1}°C",
        response.name,
        description,
        temperature,
        get_temperature_emoji(temperature),
        humidity,
        pressure,
        wind_speed,
        temp_min,
        temp_max
        
    )
}

fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature < 10.0 {
        "☁️"
    } else if temperature < 20.0 {
        "⛅"
    } else if temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = WeatherApp::new()?;
    let weak_ui = ui.as_weak();

    ui.on_get_weather(move |city, country| {
        let weak_ui_clone = weak_ui.clone();
        let city = city.to_string();
        let country = country.to_string();

        std::thread::spawn(move || {
            let api_key = "e961b2316049d0ffe66d925ae443fc7e";
            let info_text = match get_weather_info(&city, &country, api_key) {
                Ok(response) => format_weather_info(&response),
                Err(err) => format!("❌ Failed to fetch weather data: {}", err),
            };

            
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = weak_ui_clone.upgrade() {
                    ui.set_weather_text(slint::SharedString::from(info_text));
                }
            })
            .unwrap();
        });
    });

    ui.run()
}
