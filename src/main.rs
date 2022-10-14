use clap::Parser;
use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT},
};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    ip: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ResponseAPI {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    org: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let url = format!("https://ipinfo.io/{}/json", args.ip);

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36")
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {
            let response: ResponseAPI = res.json().await.unwrap();
            println!("IP: {:?}", response.ip.as_deref().unwrap_or(""));
            println!("Hostname: {:?}", response.hostname.as_deref().unwrap_or(""));
            println!("City: {:?}", response.city.as_deref().unwrap_or(""));
            println!("Region: {:?}", response.region.as_deref().unwrap_or(""));
            println!("Country: {:?}", response.country.as_deref().unwrap_or(""));
            println!("Location: {:?}", response.loc.as_deref().unwrap_or(""));
            println!("Organization: {:?}", response.org.as_deref().unwrap_or(""));
            println!("Postal: {:?}", response.postal.as_deref().unwrap_or(""));
            println!("Timezone: {:?}", response.timezone.as_deref().unwrap_or(""));
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("Too many requests, you're being rate limited");
        }
        _ => println!("Error: {}", res.status()),
    }
}
