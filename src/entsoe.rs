use crate::{
    assets::{DocumentType, ProcessType, PsrType, AREA_CODE},
    models,
};
use eyre::Result;
use hyper::StatusCode;
use std::{env, os::unix::process};

static url: &str = "https://web-api.tp.entsoe.eu/api?";
use models::error::AcknowledgementMarketDocument as EntsoeErrorResponse;
use quick_xml::de::from_str;

#[derive(Debug, Clone)]
pub struct EntsoeClient {
    pub api_key: String,
    pub area_code: Option<AREA_CODE>,
    pub process_type: Option<ProcessType>,
    pub document_type: Option<DocumentType>,
    pub psr_type: Option<PsrType>,
}

impl EntsoeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            area_code: None,
            process_type: None,
            document_type: None,
            psr_type: None,
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
    pub fn with_document_type(mut self, document_type: DocumentType) -> Self {
        self.document_type = Some(document_type);
        self
    }
    pub fn with_psr_type(mut self, psr_type: PsrType) -> Self {
        self.psr_type = Some(psr_type);
        self
    }
    pub async fn request(self) -> String {
        let fetch_from = format!("{}{}&in_Domain={}&periodStart=201512312300&periodEnd=201612312300&securityToken={}{}{}", url, self.process_type.unwrap().add_to_url(), self.area_code.unwrap().get_area().code, self.api_key, self.document_type.unwrap().add_to_url(), self.psr_type.unwrap().add_to_url());
        println!("{}", fetch_from);
        let resp = reqwest::get(&fetch_from).await.unwrap();
        match resp.status() {
            StatusCode::OK => return resp.text().await.unwrap(),
            _ => {
                let x = resp.text().await.unwrap();
                let root: EntsoeErrorResponse = from_str(&x).unwrap();
                println!("Error: {}", root.reason.text);
            }
        };

        format!("Done").to_owned()
    }
}

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
    }
}
