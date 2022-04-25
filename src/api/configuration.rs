use std::fs;
use std::io::Write;
use text_io::read;

const STORAGE: &str = "api_keys.txt";
const PROVIDER_STORAGE: &str = "current_provider.txt";

pub fn add_api_key(provider: &str) {
    println!("Please, share your api key for this provider: ");
    let value: String = read!("{}\n");

    write_key_to_file(provider, &value, STORAGE)
}

pub fn get_api_key(provider: &str) -> String {
    get_key_from_file(provider, STORAGE)
}

fn write_key_to_file(provider: &str, value: &str, filename: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename)
        .expect("Create failed");

    let to_write = format!("\n{provider}: {value}");
    file.write_all(to_write.as_bytes()).expect("Write failed");
}

fn get_key_from_file(provider: &str, filename: &str) -> String {
    let contents =
        fs::read_to_string(filename).expect("No configuration found! Please run `confugure`");

    contents
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|string| string.split(": ").collect())
        .find(|line: &Vec<&str>| line[0] == provider)
        .expect("No such provider")[1]
        .to_string()
}

pub fn get_current_provider() -> String {
    let contents = fs::read_to_string(PROVIDER_STORAGE)
        .expect("No current provider found. Please run `configure`");

    contents
}

pub fn set_current_provider(provider: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(PROVIDER_STORAGE)
        .expect("Create failed");

    let to_write = format!("{provider}");
    file.write_all(to_write.as_bytes()).expect("Write failed");
}

#[cfg(test)]
mod tests {
    use super::{fs, get_key_from_file, write_key_to_file};

    #[test]
    fn adds_and_retrieves_api_keys() {
        write_key_to_file("provider1", "123", "tmp.txt");
        write_key_to_file("provider2", "456", "tmp.txt");

        assert_eq!(get_key_from_file("provider1", "tmp.txt"), "123");
        assert_eq!(get_key_from_file("provider2", "tmp.txt"), "456");
        fs::remove_file("tmp.txt").unwrap();
    }
}
