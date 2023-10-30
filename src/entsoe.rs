use crate::assets::{get_area, AREA_CODE};
use eyre::Result;
use std::env;

static url: &str = "https://web-api.tp.entsoe.eu/api?";

struct EntsoeClient {
    api_key: String,
    area_code: Option<AREA_CODE>,
}

impl EntsoeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            area_code: None,
        }
    }

    pub fn with_area_code(mut self, area_code: AREA_CODE) -> Self {
        self.area_code = Some(area_code);
        self
    }

    pub async fn request(self) {
        let fetch_from = day_ahead_load_url(self.area_code.unwrap());
        let resp = reqwest::get(&fetch_from).await.unwrap();
        println!("..... done {:?}", resp);
    }
}

pub fn day_ahead_load_url(area_code: AREA_CODE) -> String {
    let ENTSOE_API_KEY = env::var("ENTSOE_API_KEY").unwrap();
    let area = get_area(area_code);
    let fetch_from = format!("{}&documentType=A65&processType=A01&outBiddingZone_Domain={}&periodStart=201512312300&periodEnd=201612312300&securityToken={}", url, area.code, ENTSOE_API_KEY);
    fetch_from
}

pub async fn day_ahead_load(area_code: AREA_CODE) -> Result<()> {
    let fetch_from = day_ahead_load_url(area_code);
    let resp = reqwest::get(&fetch_from).await?;
    println!("Status: {}", resp.status());
    let body = resp.text().await?;
    println!("Body:\n\n{}", body);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_day_ahead_load() {
        let area_code = AREA_CODE::DE_50HZ;
        let request_url = day_ahead_load_url(area_code);
        let resp = reqwest::get(&request_url).await.unwrap();
        println!("..... done {:?}", resp);
    }

    #[tokio::test]
    async fn test_generator() {
        let area_code = AREA_CODE::DE_50HZ;
        let client = EntsoeClient::new("".to_string());
        client.with_area_code(area_code).request().await;
    }
}
