use serde::{Deserialize, Serialize};
use strum::Display;

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
}

impl ProcessType {
    pub fn add_to_url(&self) -> String {
        format!("processType={}", self.to_string())
    }

    pub fn description(&self) -> String {
        let named = match &self {
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
            ProcessType::A56 => "Frequency restoration reserv",
        };
        named.to_string()
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
