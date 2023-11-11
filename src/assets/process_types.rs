use serde::{Deserialize, Serialize};
use strum::Display;

use super::UriElement;

#[derive(Debug, Display, Serialize, Deserialize, Clone)]
pub enum ProcessType {
    A01,
    A02,
    A16,
    A18,
    A31,
    A32,
    A33,
    A39,
    A40,
    A46,
    A47,
    A51,
    A52,
    A56,
    A60,
    A61,
    A67,
    A68,
}

impl ProcessType {
    pub fn description(&self) -> String {
        match &self {
            ProcessType::A01 => "Day ahead",
            ProcessType::A02 => "Intra day incremental",
            ProcessType::A16 => "Realised",
            ProcessType::A18 => "Intraday total",
            ProcessType::A31 => "Week ahead",
            ProcessType::A32 => "Month ahead",
            ProcessType::A33 => "Year ahead",
            ProcessType::A39 => "Synchronisation process",
            ProcessType::A40 => "Intraday process",
            ProcessType::A46 => "Replacement reserve",
            ProcessType::A47 => "Manual frequency restoration reserve",
            ProcessType::A51 => "Automatic frequency restoration reserve",
            ProcessType::A52 => "Frequency containment reserve",
            ProcessType::A56 => "Frequency restoration reserve",
            ProcessType::A60 => "Scheduled activation mFRR",
            ProcessType::A61 => "Direct activation mFRR",
            ProcessType::A67 => "Central Selection aFRR",
            ProcessType::A68 => "Local Selection aFRR",
        }
        .to_string()
    }
}

impl UriElement for ProcessType {
    fn add_to_url(&self, params: &mut Vec<(&str, String)>, key: Option<String>) {
        params.push(("processType", self.to_string()));
    }
}

#[cfg(test)]
mod test {
    use super::ProcessType;

    #[test]
    fn test_process_type() {
        assert_eq!(ProcessType::A01.to_string(), "A01");
    }
}
