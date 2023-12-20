use crate::constants::{
    get_env, GOOGLE_SEARCH_API_KEY, GOOGLE_SEARCH_API_URL, GOOGLE_SEARCH_APP_ID,
    SEARCH_COUNT_RESULT,
};
use reqwest::{Result};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleSearchResult {
    pub kind: String,
    pub url: GoogleSearchUrl,
    pub queries: GoogleSearchQueries,
    pub context: GoogleSearchContext,
    pub searchInformation: GoogleSearchInformation,
    pub items: Vec<GoogleSearchItem>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleSearchUrl {
    pub r#type: String,
    pub template: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleSearchInformation {
    pub searchTime: f64,
    pub formattedSearchTime: String,
    pub totalResults: String,
    pub formattedTotalResults: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleSearchContext {
    pub title: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleSearchRequest {
    pub title: String,
    pub totalResults: String,
    pub searchTerms: String,
    pub count: usize,
    pub startIndex: usize,
    pub inputEncoding: String,
    pub outputEncoding: String,
    pub safe: String,
    pub cx: String,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct GoogleSearchQueries {
    pub request: Vec<GoogleSearchRequest>,
    pub nextPage: Vec<GoogleSearchRequest>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleSearchItem {
    pub kind: String,
    pub snippet: String,
    pub link: String,
    pub title: String,
}

pub async fn google_search(query: &str) -> Result<(Option<GoogleSearchResult>, String)> {
    let api_key = get_env(GOOGLE_SEARCH_API_KEY);
    if let None = api_key {
        panic!("{GOOGLE_SEARCH_API_KEY} is missing");
    }
    let api_key = api_key.unwrap();

    let app_id = get_env(GOOGLE_SEARCH_APP_ID);

    if let None = app_id {
        panic!("{GOOGLE_SEARCH_APP_ID} is missing");
    }
    let app_id = app_id.unwrap();

    let url = format!(
        "{GOOGLE_SEARCH_API_URL}?key={api_key}&cx={app_id}&q={query}&num={SEARCH_COUNT_RESULT}"
    );
  
    let body = reqwest::get(url)
    .await?
    .text()
    .await?;
    let res = serde_json::from_str::<GoogleSearchResult>(&body);
    if let Err(err) = res {
        log::error!("Failed to parse response {:?}", err);
        return Ok((None, body));
    }
    let res = res.unwrap();

    Ok((Some(res), body))
}
