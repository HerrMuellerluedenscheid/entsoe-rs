use crate::{
    assets::{DocumentType, ProcessType, PsrType, UriElement, AREA_CODE},
    models::AcknowledgementMarketDocument,
};

use eyre::Result;
use quick_xml::de::from_str;

use url::Url;

static BASE_URL: &str = "https://web-api.tp.entsoe.eu/api?";

#[derive(Debug, Clone, Default)]
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

    fn get_url(self) -> Url {
        let mut params = vec![("securityToken", self.api_key)];
        params.push(("periodStart", "201512312300".to_owned()));
        params.push(("periodEnd", "201612312300".to_owned()));

        // Removed unused variable
        if let Some(process_type) = self.process_type {
            process_type.add_to_url(&mut params);
        }

        if let Some(psr_type) = self.psr_type {
            psr_type.add_to_url(&mut params);
        }

        if let Some(area_code) = self.area_code {
            area_code.add_to_url(&mut params);
        }

        if let Some(document_type) = self.document_type {
            document_type.add_to_url(&mut params);
        }

        log::info!("params {:?}", params);
        Url::parse_with_params(BASE_URL, &params).expect("failed parsing url")
    }

    pub async fn request(self) -> Result<String> {
        let url = self.get_url();
        println!("fetch from {}", url);
        let resp = reqwest::get(url).await?.error_for_status();
        match resp {
            Ok(resp) => {
                let body = resp.text().await?;
                let entsoe_response: AcknowledgementMarketDocument = from_str(&body).unwrap();
                println!("{:?}", entsoe_response.reason.text);
                return Ok(body);
            }
            Err(e) => Err(e.into()),
        }
        // let status = &resp.status();
        // let body = resp.text().await?;
        // let entsoe_response: EntsoeErrorResponse = from_str(&body).unwrap();
        // let entsoe_code = entsoe_response.reason.code.as_str();
        // let entsoe_reason = entsoe_response.reason.text;
        // let result = match (status, entsoe_code) {
        //     (&http::StatusCode::OK, "") => {
        //         return Ok(body)
        //     }
        //     (&http::StatusCode::OK, "999") => {
        //         return Err(EntsoeError::NoMatchingDataFound(entsoe_reason).into())
        //     }
        //     (&http::StatusCode::BAD_REQUEST, "999") => {
        //         return Err(EntsoeError::InvalidQueryAttributesOrParameters(entsoe_reason).into())
        //     }
        //     (_, _) => {
        //         return Err(resp.error_for_status().unwrap_err().into())
        //     }
        // };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        let client = EntsoeClient::new("api_key".to_string());
        let url = client
            .with_area_code(AREA_CODE::DE_50HZ)
            .with_process_type(ProcessType::A31)
            .get_url();
        println!("url {:?}", url);
    }
}
