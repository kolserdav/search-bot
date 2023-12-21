use dotenvy::dotenv;
use std::env;

pub const GOOGLE_SEARCH_API_KEY: &'static str = "GOOGLE_SEARCH_API_KEY";
pub const ALLOWED_USERS: &'static str = "ALLOWED_USERS";
pub const GOOGLE_SEARCH_APP_ID: &'static str = "GOOGLE_SEARCH_APP_ID";
pub const GOOGLE_SEARCH_API_URL: &'static str = "https://www.googleapis.com/customsearch/v1";
pub const SEARCH_COUNT_RESULT: u8 = 10;

pub fn dotenv_load() {
    dotenv().expect(".env file not found");
}

pub fn get_env(name: &'static str) -> Option<String> {
    let mut res: Option<String> = None;
    for (key, value) in env::vars() {
        if key == name {
            res = Some(value);
        }
    }
    res
}
