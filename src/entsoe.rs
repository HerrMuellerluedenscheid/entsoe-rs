use crate::assets::{get_area, AREA_CODE};

use eyre::Result;

static url: &str = "https://web-api.tp.entsoe.eu/api?";

pub fn day_ahead_load_url(area_code: AREA_CODE) -> String {
    let area = get_area(area_code);
    let fetch_from = format!("{}documentType=A65&processType=A01&outBiddingZone_Domain={}&periodStart=201512312300&periodEnd=201612312300", url, area.code);
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
}
