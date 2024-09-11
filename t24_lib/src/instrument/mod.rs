use serde_repr::{Deserialize_repr, Serialize_repr};
use near_sdk::near;
use t24_macros::EnumI64Derive;

#[derive(Hash,Debug, Clone, Copy, Serialize_repr, Deserialize_repr, EnumI64Derive, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "std", derive(clap::ValueEnum))]
#[near(serializers = [borsh])]
#[repr(u8)]
pub enum Instrument {
    AudCad,
    AudChf,
    AudHkd,
    AudJpy,
    AudNzd,
    AudSgd,
    AudUsd,
    CadChf,
    CadHkd,
    CadJpy,
    CadSgd,
    ChfHkd,
    ChfJpy,
    EurAud,
    EurCad,
    EurChf,
    EurGbp,
    EurHkd,
    EurJpy,
    EurNzd,
    EurSgd,
    EurUsd,
    GbpAud,
    GbpCad,
    GbpChf,
    GbpHkd,
    GbpJpy,
    GbpNzd,
    GbpSgd,
    GbpUsd,
    HkdJpy,
    NzdCad,
    NzdChf,
    NzdHkd,
    NzdJpy,
    NzdSgd,
    NzdUsd,
    SgdChf,
    SgdHkd,
    SgdJpy,
    UsdCad,
    UsdChf,
    UsdHkd,
    UsdJpy,
    UsdSgd,
    Jpy,
    NonFarmUsPayroll,
    Usd,
    Eur
}

impl Instrument {
    pub fn pip_i64(&self)->i64{
        match self {
            Instrument::EurUsd => 1,
            Instrument::UsdJpy => 100,
            _ => panic!()
        }
    }

}