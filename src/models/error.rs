// Generated with crate xml_schema_generator
// cargo run --features="env_logger" -- input.xml

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Acknowledgement_MarketDocument")]
    pub acknowledgement_market_document: AcknowledgementMarketDocument,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcknowledgementMarketDocument {
    #[serde(rename = "mRID")]
    pub m_rid: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "sender_MarketParticipant.marketRole.type")]
    pub sender_market_participant_market_role_type: String,
    #[serde(rename = "receiver_MarketParticipant.marketRole.type")]
    pub receiver_market_participant_market_role_type: String,
    #[serde(rename = "received_MarketDocument.createdDateTime")]
    pub received_market_document_created_date_time: String,
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "sender_MarketParticipant.mRID")]
    pub sender_market_participant_m_rid: SenderMarketParticipantMRID,
    #[serde(rename = "receiver_MarketParticipant.mRID")]
    pub receiver_market_participant_m_rid: ReceiverMarketParticipantMRID,
    #[serde(rename = "Reason")]
    pub reason: Reason,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SenderMarketParticipantMRID {
    #[serde(rename = "@codingScheme")]
    pub coding_scheme: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiverMarketParticipantMRID {
    #[serde(rename = "@codingScheme")]
    pub coding_scheme: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reason {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "text")]
    pub text: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_xml_deserialization() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Acknowledgement_MarketDocument
                xmlns="urn:iec62325.351:tc57wg16:451-1:acknowledgementdocument:7:0">
                <mRID>648af177-b3de-4</mRID>
                <createdDateTime>2023-11-04T09:23:46Z</createdDateTime>
        
                <sender_MarketParticipant.mRID codingScheme="A01">10X1001A1001A450</sender_MarketParticipant.mRID>
                <sender_MarketParticipant.marketRole.type>A32</sender_MarketParticipant.marketRole.type>
        
                <receiver_MarketParticipant.mRID codingScheme="A01">10X1001A1001A450</receiver_MarketParticipant.mRID>
                <receiver_MarketParticipant.marketRole.type>A39</receiver_MarketParticipant.marketRole.type>
        
        
                <received_MarketDocument.createdDateTime>2023-11-04T09:23:46Z</received_MarketDocument.createdDateTime>
        
        
                <Reason>
                        <code>999</code>
                        <text>Web API request contains more then one definition of param processType</text>
                </Reason>
        
        </Acknowledgement_MarketDocument>"#;
        let root: AcknowledgementMarketDocument = from_str(xml).unwrap();
        println!("{:?}", root.reason.text);
    }
}
