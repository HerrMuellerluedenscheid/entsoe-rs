use crate::{
    assets::{DocumentType, ProcessType, PsrType, UriElement, AREA_CODE},
    error::EntsoeError,
    forecast,
    models::{AcknowledgementMarketDocument, GLMarketDocument},
};

use chrono::{DateTime, Utc};
use eyre::Result;
use quick_xml::de::from_str;
use url::Url;

type DateType = DateTime<chrono::Utc>;
static BASE_URL: &str = "https://web-api.tp.entsoe.eu/api?";

#[derive(Debug, Clone, Default)]
pub struct EntsoeClient {
    pub api_key: String,
    pub period_start: Option<DateType>,
    pub period_end: Option<DateType>,
    pub in_domain: Option<AREA_CODE>,
    pub out_domain: Option<AREA_CODE>,
    pub in_bidding_zone_domain: Option<AREA_CODE>,
    pub out_bidding_zone_domain: Option<AREA_CODE>,
    pub process_type: Option<ProcessType>,
    pub document_type: Option<DocumentType>,
    pub psr_type: Option<PsrType>,
}

impl EntsoeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            ..Default::default()
        }
    }

    pub fn with_period_start(mut self, period_start: DateType) -> Self {
        self.period_start = Some(period_start);
        self
    }

    pub fn with_period_end(mut self, period_end: DateType) -> Self {
        self.period_end = Some(period_end);
        self
    }

    pub fn with_out_bidding_zone(mut self, out_bidding_zone: AREA_CODE) -> Self {
        self.out_bidding_zone_domain = Some(out_bidding_zone);
        self
    }

    pub fn with_in_domain(mut self, area_code: AREA_CODE) -> Self {
        self.in_domain = Some(area_code);
        self
    }

    pub fn with_out_domain(mut self, area_code: AREA_CODE) -> Self {
        self.out_domain = Some(area_code);
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

        if let Some(period_start) = self.period_start {
            params.push(("periodStart", formatted_datetime(&period_start)));
        }

        if let Some(period_end) = self.period_end {
            params.push(("periodEnd", formatted_datetime(&period_end)));
        }

        if let Some(process_type) = self.process_type {
            process_type.add_to_url(&mut params, None);
        }

        if let Some(psr_type) = self.psr_type {
            psr_type.add_to_url(&mut params, None);
        }

        if let Some(out_domain) = self.out_domain {
            params.push(("out_Domain", out_domain.get_area().code.to_string()));
        }

        if let Some(in_domain) = self.in_domain {
            params.push(("in_Domain", in_domain.get_area().code.to_string()));
        }

        if let Some(out_bidding_zone_domain) = self.out_bidding_zone_domain {
            params.push((
                "outBiddingZone_Domain",
                out_bidding_zone_domain.get_area().code.to_string(),
            ));
        }

        if let Some(in_bidding_zone_domain) = self.in_bidding_zone_domain {
            params.push((
                "inBiddingZone_Domain",
                in_bidding_zone_domain.get_area().code.to_string(),
            ));
        }

        if let Some(document_type) = self.document_type {
            document_type.add_to_url(&mut params, None);
        }

        log::info!("params {:?}", params);
        Url::parse_with_params(BASE_URL, &params).expect("failed parsing url")
    }

    pub async fn request(self) -> Result<GLMarketDocument> {
        let url = self.get_url();
        println!("fetch from {}", url);
        let resp = reqwest::get(url).await;

        let body = resp.unwrap().text().await?;
        let parsed: GLMarketDocument = from_str(&body)?;
        Ok(parsed)
        // match resp {
        //     Ok(resp) => {
        //         let body = resp.text().await?;
        //         println!("body {:?}", body);
        //         let entsoe_response: AcknowledgementMarketDocument = from_str(&body).unwrap();
        //         match entsoe_response.reason.code.as_str() {
        //             "" => return Ok(body),
        //             "999" => {
        //                 return Err(
        //                     EntsoeError::NoMatchingDataFound(entsoe_response.reason.text).into(),
        //                 )
        //             }
        //             _ => {
        //                 return Err(EntsoeError::InvalidQueryAttributesOrParameters(
        //                     entsoe_response.reason.text,
        //                 )
        //                 .into())
        //             }
        //         }
        //     }
        //     Err(e) => Err(e.into()),
        // }
    }
}

fn formatted_datetime(period_start: &DateTime<Utc>) -> String {
    format!("{}", period_start.format("%Y%m%d%H%M"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use dotenvy::dotenv;
    use serde::{Deserialize, Serialize};

    #[tokio::test]
    async fn test_get_url() {
        let client = EntsoeClient::new("api_key".to_string());
        let url = client
            .with_in_domain(AREA_CODE::DE_50HZ)
            .with_process_type(ProcessType::A31)
            .get_url();
        println!("url {:?}", url);
    }

    #[tokio::test]
    async fn test_fetch_week_ahead() {
        dotenv().unwrap();
        let entsoe_api_key =
            std::env::var("ENTSOE_API_KEY").expect("ENTSOE_API_KEY is undefined env var");
        let client = EntsoeClient::new(entsoe_api_key);
        let start = chrono::Utc
            .with_ymd_and_hms(2015, 12, 31, 23, 0, 0)
            .unwrap();
        let end = chrono::Utc
            .with_ymd_and_hms(2016, 12, 31, 23, 0, 0)
            .unwrap();
        let result = client
            .with_period_start(start)
            .with_period_end(end)
            .with_out_bidding_zone(AREA_CODE::DE_50HZ)
            .with_process_type(ProcessType::A31)
            .with_document_type(DocumentType::A65)
            .request()
            .await;
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_deserialize_datetime() {
        #[derive(Debug, Deserialize, Serialize)]
        struct Test {
            pub period_start: DateType,
        }

        let t = Test {
            period_start: chrono::Utc
                .with_ymd_and_hms(2015, 12, 31, 23, 0, 0)
                .unwrap(),
        };

        println!("test as json: {:?}", serde_json::to_string(&t).unwrap());
    }

    #[tokio::test]
    async fn test_fetch_day_ahead() {
        // GET /api?documentType=A69&processType=A01&psrType=B16&in_Domain=10YCZ-CEPS-----N&periodStart=201512312300&periodEnd=201612312300
        dotenv().unwrap();
        let entsoe_api_key =
            std::env::var("ENTSOE_API_KEY").expect("ENTSOE_API_KEY is undefined env var");
        let client = EntsoeClient::new(entsoe_api_key);
        let start = chrono::Utc
            .with_ymd_and_hms(2015, 12, 30, 23, 0, 0)
            .unwrap();
        let end = chrono::Utc
            .with_ymd_and_hms(2015, 12, 31, 23, 0, 0)
            .unwrap();
        println!("start {:?}", start.format("%Y%m%d%H%M"));
        let result = client
            .with_period_start(start)
            .with_period_end(end)
            .with_in_domain(AREA_CODE::DE_50HZ)
            .with_process_type(ProcessType::A01) // day ahead
            .with_document_type(DocumentType::A69)
            .with_psr_type(PsrType::B16)
            .request()
            .await;
        // println!("{:?}", result);
    }
}
