use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GLMarketDocument {
    #[serde(rename = "mRID")]
    pub m_rid: String,
    #[serde(rename = "revisionNumber")]
    pub revision_number: String,
    #[serde(rename = "type")]
    pub gl_market_document_type: String,
    #[serde(rename = "process.processType")]
    pub process_process_type: String,
    #[serde(rename = "sender_MarketParticipant.marketRole.type")]
    pub sender_market_participant_market_role_type: String,
    #[serde(rename = "receiver_MarketParticipant.marketRole.type")]
    pub receiver_market_participant_market_role_type: String,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "sender_MarketParticipant.mRID")]
    pub sender_market_participant_m_rid: SenderMarketParticipantMRID,
    #[serde(rename = "receiver_MarketParticipant.mRID")]
    pub receiver_market_participant_m_rid: ReceiverMarketParticipantMRID,
    #[serde(rename = "time_Period.timeInterval")]
    pub time_period_time_interval: TimePeriodTimeInterval,
    #[serde(rename = "TimeSeries")]
    pub time_series: Vec<TimeSeries>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SenderMarketParticipantMRID {
    #[serde(rename = "@codingScheme")]
    pub coding_scheme: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReceiverMarketParticipantMRID {
    #[serde(rename = "@codingScheme")]
    pub coding_scheme: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimePeriodTimeInterval {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimeSeries {
    #[serde(rename = "mRID")]
    pub m_rid: String,
    #[serde(rename = "businessType")]
    pub business_type: String,
    #[serde(rename = "objectAggregation")]
    pub object_aggregation: String,
    #[serde(rename = "quantity_Measure_Unit.name")]
    pub quantity_measure_unit_name: String,
    #[serde(rename = "curveType")]
    pub curve_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "inBiddingZone_Domain.mRID")]
    pub in_bidding_zone_domain_m_rid: InBiddingZoneDomainMRID,
    #[serde(rename = "MktPSRType")]
    pub mkt_psrtype: MktPSRType,
    #[serde(rename = "Period")]
    pub period: Period,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InBiddingZoneDomainMRID {
    #[serde(rename = "@codingScheme")]
    pub coding_scheme: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MktPSRType {
    #[serde(rename = "psrType")]
    pub psr_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Period {
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "timeInterval")]
    pub time_interval: TimeInterval,
    #[serde(rename = "Point")]
    pub point: Vec<Point>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TimeInterval {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Point {
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
