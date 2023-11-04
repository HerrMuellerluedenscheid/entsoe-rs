use crate::assets::{get_area, ProcessType, AREA_CODE};
use eyre::Result;
use std::{env, os::unix::process};

static url: &str = "https://web-api.tp.entsoe.eu/api?";

#[derive(Debug, Clone)]
pub struct EntsoeClient {
    pub api_key: String,
    pub area_code: Option<AREA_CODE>,
    pub process_type: Option<ProcessType>,
}

impl EntsoeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            area_code: None,
            process_type: None,
        }
    }

    pub fn with_area_code(mut self, area_code: AREA_CODE) -> Self {
        self.area_code = Some(area_code);
        self
    }

    pub fn with_process_type(mut self, process_type: ProcessType) -> Self {
        self.process_type = Some(process_type);
        self
    }
    pub async fn request(self) -> String {
        let fetch_from = format!("{}&documentType=A65&processType=A01&outBiddingZone_Domain={}&periodStart=201512312300&periodEnd=201612312300&{}&securityToken={}", url, self.area_code.unwrap(), self.process_type.unwrap().add_to_url(), self.api_key);
        // let resp = reqwest::get(&fetch_from).await;
        // resp.unwrap().text().await.unwrap()
        "Done".to_owned()
    }
}

// pub fn day_ahead_load_url(area_code: AREA_CODE, process_type: ProcessType) -> String {
//     // let ENTSOE_API_KEY = env::var("ENTSOE_API_KEY").unwrap();
//     let area = get_area(area_code);
//     let fetch_from = format!("{}&documentType=A65&processType=A01&outBiddingZone_Domain={}&periodStart=201512312300&periodEnd=201612312300&{}&securityToken={}", url, area.code, process_type.add_url(), ENTSOE_API_KEY);
//     fetch_from
// }

// pub async fn day_ahead_load(area_code: AREA_CODE, process_type: ProcessType) -> Result<String> {
//     let fetch_from = day_ahead_load_url(area_code, process_type);
//     let resp = reqwest::get(&fetch_from).await?;
//     println!("Status: {}", resp.status());
//     let body = resp.text().await?;
//     Ok(body)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_day_ahead_load() {
        let area_code = AREA_CODE::DE_50HZ;
        let process_type = ProcessType::A01;
    }

    #[tokio::test]
    async fn test_generator() {
        let area_code = AREA_CODE::DE_50HZ;
        let client = EntsoeClient::new("".to_string());
        client.with_area_code(area_code).request().await;
    }
}
