use std::fmt;
use std::str::FromStr;
use crate::error::conversion;

#[derive(Debug, Clone, PartialEq)]
pub enum CpuType {
    Epyc,
    EpycV1,
    EpycV2,
    EpycIBPB,
    EpycV3,
    EpycV4,
    EpycRome,
    EpycRomeV1,
    EpycRomeV2,
    EpycRomeV3,
    EpycMilan,
    EpycMilanV1,
    EpycMilanV2,
}

impl CpuType {
    pub fn sig(&self) -> i32 {
        match self {
            CpuType::Epyc => cpu_sig(23, 1, 2),
            CpuType::EpycV1 => cpu_sig(23, 1, 2),
            CpuType::EpycV2 => cpu_sig(23, 1, 2),
            CpuType::EpycIBPB => cpu_sig(23, 1, 2),
            CpuType::EpycV3 => cpu_sig(23, 1, 2),
            CpuType::EpycV4 => cpu_sig(23, 1, 2),
            CpuType::EpycRome => cpu_sig(23, 49, 0),
            CpuType::EpycRomeV1 => cpu_sig(23, 49, 0),
            CpuType::EpycRomeV2 => cpu_sig(23, 49, 0),
            CpuType::EpycRomeV3 => cpu_sig(23, 49, 0),
            CpuType::EpycMilan => cpu_sig(25, 1, 1),
            CpuType::EpycMilanV1 => cpu_sig(25, 1, 1),
            CpuType::EpycMilanV2 => cpu_sig(25, 1, 1),
        }
    }
}

impl fmt::Display for CpuType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuType::Epyc => write!(f, "EPYC"),
            CpuType::EpycV1 => write!(f, "EPYC-v1"),
            CpuType::EpycV2 => write!(f, "EPYC-v2"),
            CpuType::EpycIBPB => write!(f, "EPYC-IBPB"),
            CpuType::EpycV3 => write!(f, "EPYC-v3"),
            CpuType::EpycV4 => write!(f, "EPYC-v4"),
            CpuType::EpycRome => write!(f, "EPYC-Rome"),
            CpuType::EpycRomeV1 => write!(f, "EPYC-Rome-v1"),
            CpuType::EpycRomeV2 => write!(f, "EPYC-Rome-v2"),
            CpuType::EpycRomeV3 => write!(f, "EPYC-Rome-v3"),
            CpuType::EpycMilan => write!(f, "EPYC-Milan"),
            CpuType::EpycMilanV1 => write!(f, "EPYC-Milan-v1"),
            CpuType::EpycMilanV2 => write!(f, "EPYC-Milan-v2"),
        }
    }
}

impl FromStr for CpuType {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EPYC" => Ok(CpuType::Epyc),
            "EPYC-v1" => Ok(CpuType::EpycV1),
            "EPYC-v2" => Ok(CpuType::EpycV2),
            "EPYC-IBPB" => Ok(CpuType::EpycIBPB),
            "EPYC-v3" => Ok(CpuType::EpycV3),
            "EPYC-v4" => Ok(CpuType::EpycV4),
            "EPYC-Rome" => Ok(CpuType::EpycRome),
            "EPYC-Rome-v1" => Ok(CpuType::EpycRomeV1),
            "EPYC-Rome-v2" => Ok(CpuType::EpycRomeV2),
            "EPYC-Rome-v3" => Ok(CpuType::EpycRomeV3),
            "EPYC-Milan" => Ok(CpuType::EpycMilan),
            "EPYC-Milan-v1" => Ok(CpuType::EpycMilanV1),
            "EPYC-Milan-v2" => Ok(CpuType::EpycMilanV2),
            _ => Err(conversion(format!("unable to create CpuType from_str: {}", s), None)),
        }
    }
}

/// Compute the 32-bit CPUID signature from family, model, and stepping.
///
/// This computation is described in AMD's CPUID Specification, publication #25481
/// https://www.amd.com/system/files/TechDocs/25481.pdf
/// See section: CPUID Fn0000_0001_EAX Family, Model, Stepping Identifiers
pub fn cpu_sig(family: i32, model: i32, stepping: i32) -> i32 {
    let family_low;
    let family_high;

    if family > 0xf {
        family_low = 0xf;
        family_high = (family - 0x0f) & 0xff;
    } else {
        family_low = family;
        family_high = 0;
    }

    let model_low = model & 0xf;
    let model_high = (model >> 4) & 0xf;

    let stepping_low = stepping & 0xf;

    (family_high << 20) |
        (model_high << 16) |
        (family_low << 8) |
        (model_low << 4) |
        stepping_low
}