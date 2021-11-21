#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCU Status Register"]
    pub pwrstat: crate::Reg<pwrstat::PWRSTAT_SPEC>,
    #[doc = "0x04 - PCU Set Control Register"]
    pub pwrset: crate::Reg<pwrset::PWRSET_SPEC>,
    #[doc = "0x08 - PCU Clear Control Register"]
    pub pwrclr: crate::Reg<pwrclr::PWRCLR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - EVR Status Register"]
    pub evrstat: crate::Reg<evrstat::EVRSTAT_SPEC>,
    #[doc = "0x14 - EVR VADC Status Register"]
    pub evrvadcstat: crate::Reg<evrvadcstat::EVRVADCSTAT_SPEC>,
    _reserved5: [u8; 0x14],
    #[doc = "0x2c - Power Monitor Control"]
    pub pwrmon: crate::Reg<pwrmon::PWRMON_SPEC>,
}
#[doc = "PWRSTAT register accessor: an alias for `Reg<PWRSTAT_SPEC>`"]
pub type PWRSTAT = crate::Reg<pwrstat::PWRSTAT_SPEC>;
#[doc = "PCU Status Register"]
pub mod pwrstat;
#[doc = "PWRSET register accessor: an alias for `Reg<PWRSET_SPEC>`"]
pub type PWRSET = crate::Reg<pwrset::PWRSET_SPEC>;
#[doc = "PCU Set Control Register"]
pub mod pwrset;
#[doc = "PWRCLR register accessor: an alias for `Reg<PWRCLR_SPEC>`"]
pub type PWRCLR = crate::Reg<pwrclr::PWRCLR_SPEC>;
#[doc = "PCU Clear Control Register"]
pub mod pwrclr;
#[doc = "EVRSTAT register accessor: an alias for `Reg<EVRSTAT_SPEC>`"]
pub type EVRSTAT = crate::Reg<evrstat::EVRSTAT_SPEC>;
#[doc = "EVR Status Register"]
pub mod evrstat;
#[doc = "EVRVADCSTAT register accessor: an alias for `Reg<EVRVADCSTAT_SPEC>`"]
pub type EVRVADCSTAT = crate::Reg<evrvadcstat::EVRVADCSTAT_SPEC>;
#[doc = "EVR VADC Status Register"]
pub mod evrvadcstat;
#[doc = "PWRMON register accessor: an alias for `Reg<PWRMON_SPEC>`"]
pub type PWRMON = crate::Reg<pwrmon::PWRMON_SPEC>;
#[doc = "Power Monitor Control"]
pub mod pwrmon;
