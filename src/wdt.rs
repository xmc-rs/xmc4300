#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT ID Register"]
    pub id: ID,
    #[doc = "0x04 - WDT Control Register"]
    pub ctr: CTR,
    #[doc = "0x08 - WDT Service Register"]
    pub srv: SRV,
    #[doc = "0x0c - WDT Timer Register"]
    pub tim: TIM,
    #[doc = "0x10 - WDT Window Lower Bound Register"]
    pub wlb: WLB,
    #[doc = "0x14 - WDT Window Upper Bound Register"]
    pub wub: WUB,
    #[doc = "0x18 - WDT Status Register"]
    pub wdtsts: WDTSTS,
    #[doc = "0x1c - WDT Clear Register"]
    pub wdtclr: WDTCLR,
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "WDT ID Register"]
pub mod id;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "WDT Control Register"]
pub mod ctr;
#[doc = "SRV (w) register accessor: an alias for `Reg<SRV_SPEC>`"]
pub type SRV = crate::Reg<srv::SRV_SPEC>;
#[doc = "WDT Service Register"]
pub mod srv;
#[doc = "TIM (r) register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "WDT Timer Register"]
pub mod tim;
#[doc = "WLB (rw) register accessor: an alias for `Reg<WLB_SPEC>`"]
pub type WLB = crate::Reg<wlb::WLB_SPEC>;
#[doc = "WDT Window Lower Bound Register"]
pub mod wlb;
#[doc = "WUB (rw) register accessor: an alias for `Reg<WUB_SPEC>`"]
pub type WUB = crate::Reg<wub::WUB_SPEC>;
#[doc = "WDT Window Upper Bound Register"]
pub mod wub;
#[doc = "WDTSTS (r) register accessor: an alias for `Reg<WDTSTS_SPEC>`"]
pub type WDTSTS = crate::Reg<wdtsts::WDTSTS_SPEC>;
#[doc = "WDT Status Register"]
pub mod wdtsts;
#[doc = "WDTCLR (w) register accessor: an alias for `Reg<WDTCLR_SPEC>`"]
pub type WDTCLR = crate::Reg<wdtclr::WDTCLR_SPEC>;
#[doc = "WDT Clear Register"]
pub mod wdtclr;
