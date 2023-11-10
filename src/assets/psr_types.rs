use serde::{Deserialize, Serialize};
use strum::Display;

use super::UriElement;

#[derive(Display, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PsrType {
    A03,
    A04,
    A05,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
    B11,
    B12,
    B13,
    B14,
    B15,
    B16,
    B17,
    B18,
    B19,
    B20,
    B21,
    B22,
    B23,
    B24,
}

impl PsrType {
    pub fn description(self) -> String {
        match self {
            PsrType::A03 => "Mixed",
            PsrType::A04 => "Generation",
            PsrType::A05 => "Load",
            PsrType::B01 => "Biomass",
            PsrType::B02 => "Fossil Brown coal/Lignite   ",
            PsrType::B03 => "Fossil Coal-derived gas",
            PsrType::B04 => "Fossil Gas",
            PsrType::B05 => "Fossil Hard coal",
            PsrType::B06 => "Fossil Oil",
            PsrType::B07 => "Fossil Oil shale",
            PsrType::B08 => "Fossil Peat",
            PsrType::B09 => "Geothermal",
            PsrType::B10 => "Hydro Pumped Storage",
            PsrType::B11 => "Hydro Run-of-river and poundage",
            PsrType::B12 => "Hydro Water Reservoir",
            PsrType::B13 => "Marine",
            PsrType::B14 => "Nuclear",
            PsrType::B15 => "Other renewable",
            PsrType::B16 => "Solar",
            PsrType::B17 => "Waste",
            PsrType::B18 => "Wind Offshore",
            PsrType::B19 => "Wind Onshore",
            PsrType::B20 => "Other",
            PsrType::B21 => "AC Link",
            PsrType::B22 => "DC Link",
            PsrType::B23 => "Substation",
            PsrType::B24 => "Transformer ",
        }
        .to_string()
    }
}

impl UriElement for PsrType {
    fn add_to_url(&self, params: &mut Vec<(&str, String)>) {
        params.push(("psrType", self.to_string()));
    }
}
