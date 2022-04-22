use super::rps::ApiResponse;
use reqwest;

const OPENWEATHER_BASE_URL: &str = "https://api.openweathermap.org/data/2.5";
const WEATHERAPI_BASE_URL: &str = "http://api.weatherapi.com/v1";

pub fn path(provider: &str, address: &str, api_key: String) -> String {
    match provider {
        "openweather" => {
            format!("{OPENWEATHER_BASE_URL}/weather?q={address}&appid={api_key}&units=metric")
        }
        "weatherapi" => format!("{WEATHERAPI_BASE_URL}/current.json?q={address}&key={api_key}"),
        _ => panic!("No such provider"),
    }
}

pub async fn get_weather(provider: &str, address: &str, api_key: String) {
    let url = path(provider, address, api_key);

    let result = reqwest::get(url).await.expect("Something went wrong");

    let response = match result.status() {
        reqwest::StatusCode::OK => result.json::<ApiResponse>().await,
        reqwest::StatusCode::UNAUTHORIZED => panic!("Wrong api key"),
        _ => panic!("Something went wrong"),
    };

    println!(
        "Current weather in {} is {:?}",
        address,
        response.unwrap().temp()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_resolves_correctly_with_openweather() {
        assert_eq!(
            path("openweather", "Kyiv", "key".to_string()),
            format!("{OPENWEATHER_BASE_URL}/weather?q=Kyiv&appid=key&units=metric")
        )
    }

    #[test]
    fn path_resolves_correctly_with_weatherapi() {
        assert_eq!(
            path("weatherapi", "Kyiv", "key".to_string()),
            format!("{WEATHERAPI_BASE_URL}/current.json?q=Kyiv&key=key")
        )
    }

    #[test]
    #[should_panic]
    fn path_panics_with_wrong_api_provider() {
        path("other_provider", "Kyiv", "key".to_string());
    }
}
