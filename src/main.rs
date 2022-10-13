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

#[derive(Serialize, Deserialize, Debug)]
struct ResponseAPI {
    ip: String,
    hostname: String,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    postal: String,
    timezone: String,
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
            println!("IP: {}", response.ip);
            println!("Hostname: {}", response.hostname);
            println!("City: {}", response.city);
            println!("Region: {}", response.region);
            println!("Country: {}", response.country);
            println!("Location: {}", response.loc);
            println!("Organization: {}", response.org);
            println!("Postal: {}", response.postal);
            println!("Timezone: {}", response.timezone);
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("Too many requests, you're being rate limited");
        }
        _ => println!("Error: {}", res.status()),
    }
}
