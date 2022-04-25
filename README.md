# Simple Rust CLI application to get the weather

Supported providers: 
- [OpenWeather](https://openweathermap.org/)
- [WeatherAPI](https://www.weatherapi.com/)

## Installation:

Build the project:

```bash
cargo build --release
```

Symlink(optional):

```bash
ln -s target/release/elastio-test weather
```

Call with symlink:
```bash
./weather <commends>
```

Call without symlink:
```bash
target/release/elastio-test <commands>

# or

cargo run <commands> # will compile and run the project
```

## Usage

### Configure:

This allows the application to store your keys in `api_keys.txt` file, that later will be parsed to get your keys.

```bash
weather configure <provider> # provider is one of the supported providers
```

### Get the weather

```bash
weather get <provider> <address> # address is either a city or a country
```
On the first run you **must** include the provider, it will remember the provider you chose(by writing it into `current_provider.txt` file), so that later you can omit the provider like so:


```bash
weather get <address> # address is either a city or a country
```

If you want to get the weather with a different provider, just specify it again, the `current_provider.txt` will be rewritten.
