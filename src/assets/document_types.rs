use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DocumentType {
    A09,
    A11,
    A15,
    A24,
    A25,
    A26,
    A31,
    A37,
    A38,
    A44,
    A61,
    A63,
    A65,
    A68,
    A69,
    A70,
    A71,
    A72,
    A73,
    A74,
    A75,
    A76,
    A77,
    A78,
    A79,
    A80,
    A81,
    A82,
    A83,
    A84,
    A85,
    A86,
    A87,
    A88,
    A89,
    A90,
    A91,
    A92,
    A93,
    A94,
    A95,
    B11,
    B17,
    B45,
}

impl DocumentType {
    pub fn description(&self) -> String {
        let description = match self {
            DocumentType::A09 => "Finalised schedule",
            DocumentType::A11 => "Aggregated energy data report",
            DocumentType::A15 => "Acquiring system operator reserve schedule",
            DocumentType::A24 => "Bid document",
            DocumentType::A25 => "Allocation result document",
            DocumentType::A26 => "Capacity document",
            DocumentType::A31 => "Agreed capacity",
            DocumentType::A37 => "Reserve bid document",
            DocumentType::A38 => "Reserve allocation result document",
            DocumentType::A44 => "Price Document",
            DocumentType::A61 => "Estimated Net Transfer Capacity",
            DocumentType::A63 => "Redispatch notice",
            DocumentType::A65 => "System total load",
            DocumentType::A68 => "Installed generation per type",
            DocumentType::A69 => "Wind and solar forecast",
            DocumentType::A70 => "Load forecast margin",
            DocumentType::A71 => "Generation forecast",
            DocumentType::A72 => "Reservoir filling information",
            DocumentType::A73 => "Actual generation",
            DocumentType::A74 => "Wind and solar generation",
            DocumentType::A75 => "Actual generation per type",
            DocumentType::A76 => "Load unavailability",
            DocumentType::A77 => "Production unavailability",
            DocumentType::A78 => "Transmission unavailability",
            DocumentType::A79 => "Offshore grid infrastructure unavailability",
            DocumentType::A80 => "Generation unavailability",
            DocumentType::A81 => "Contracted reserves",
            DocumentType::A82 => "Accepted offers",
            DocumentType::A83 => "Activated balancing quantities",
            DocumentType::A84 => "Activated balancing prices",
            DocumentType::A85 => "Imbalance prices",
            DocumentType::A86 => "Imbalance volume",
            DocumentType::A87 => "Financial situation",
            DocumentType::A88 => "Cross border balancing",
            DocumentType::A89 => "Contracted reserve prices",
            DocumentType::A90 => "Interconnection network expansion",
            DocumentType::A91 => "Counter trade notice",
            DocumentType::A92 => "Congestion costs",
            DocumentType::A93 => "DC link capacity",
            DocumentType::A94 => "Non EU allocations",
            DocumentType::A95 => "Configuration document",
            DocumentType::B11 => "Flow-based allocations",
            DocumentType::B17 => "Aggregated netted external TSO schedule document",
            DocumentType::B45 => "Bid Availability Document",
        };
        format!("{} ({:?})", description, self)
    }

    pub fn add_to_url(self) -> String {
        format!("&documentType={:?}", self)
    }
}

#[cfg(test)]
mod tests {

    use super::DocumentType;

    #[test]
    fn test_document_type() {
        assert_eq!(DocumentType::A09.description(), "Finalised schedule (A09)");
    }
}
