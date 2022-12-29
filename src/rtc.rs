#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC ID Register"]
    pub id: ID,
    #[doc = "0x04 - RTC Control Register"]
    pub ctr: CTR,
    #[doc = "0x08 - RTC Raw Service Request Register"]
    pub rawstat: RAWSTAT,
    #[doc = "0x0c - RTC Service Request Status Register"]
    pub stssr: STSSR,
    #[doc = "0x10 - RTC Service Request Mask Register"]
    pub msksr: MSKSR,
    #[doc = "0x14 - RTC Clear Service Request Register"]
    pub clrsr: CLRSR,
    #[doc = "0x18 - RTC Alarm Time Register 0"]
    pub atim0: ATIM0,
    #[doc = "0x1c - RTC Alarm Time Register 1"]
    pub atim1: ATIM1,
    #[doc = "0x20 - RTC Time Register 0"]
    pub tim0: TIM0,
    #[doc = "0x24 - RTC Time Register 1"]
    pub tim1: TIM1,
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "RTC ID Register"]
pub mod id;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "RTC Control Register"]
pub mod ctr;
#[doc = "RAWSTAT (r) register accessor: an alias for `Reg<RAWSTAT_SPEC>`"]
pub type RAWSTAT = crate::Reg<rawstat::RAWSTAT_SPEC>;
#[doc = "RTC Raw Service Request Register"]
pub mod rawstat;
#[doc = "STSSR (r) register accessor: an alias for `Reg<STSSR_SPEC>`"]
pub type STSSR = crate::Reg<stssr::STSSR_SPEC>;
#[doc = "RTC Service Request Status Register"]
pub mod stssr;
#[doc = "MSKSR (rw) register accessor: an alias for `Reg<MSKSR_SPEC>`"]
pub type MSKSR = crate::Reg<msksr::MSKSR_SPEC>;
#[doc = "RTC Service Request Mask Register"]
pub mod msksr;
#[doc = "CLRSR (w) register accessor: an alias for `Reg<CLRSR_SPEC>`"]
pub type CLRSR = crate::Reg<clrsr::CLRSR_SPEC>;
#[doc = "RTC Clear Service Request Register"]
pub mod clrsr;
#[doc = "ATIM0 (rw) register accessor: an alias for `Reg<ATIM0_SPEC>`"]
pub type ATIM0 = crate::Reg<atim0::ATIM0_SPEC>;
#[doc = "RTC Alarm Time Register 0"]
pub mod atim0;
#[doc = "ATIM1 (rw) register accessor: an alias for `Reg<ATIM1_SPEC>`"]
pub type ATIM1 = crate::Reg<atim1::ATIM1_SPEC>;
#[doc = "RTC Alarm Time Register 1"]
pub mod atim1;
#[doc = "TIM0 (rw) register accessor: an alias for `Reg<TIM0_SPEC>`"]
pub type TIM0 = crate::Reg<tim0::TIM0_SPEC>;
#[doc = "RTC Time Register 0"]
pub mod tim0;
#[doc = "TIM1 (rw) register accessor: an alias for `Reg<TIM1_SPEC>`"]
pub type TIM1 = crate::Reg<tim1::TIM1_SPEC>;
#[doc = "RTC Time Register 1"]
pub mod tim1;
