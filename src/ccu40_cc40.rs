#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Input Selector Configuration"]
    pub ins: crate::Reg<ins::INS_SPEC>,
    #[doc = "0x04 - Connection Matrix Control"]
    pub cmc: crate::Reg<cmc::CMC_SPEC>,
    #[doc = "0x08 - Slice Timer Status"]
    pub tcst: crate::Reg<tcst::TCST_SPEC>,
    #[doc = "0x0c - Slice Timer Run Set"]
    pub tcset: crate::Reg<tcset::TCSET_SPEC>,
    #[doc = "0x10 - Slice Timer Clear"]
    pub tcclr: crate::Reg<tcclr::TCCLR_SPEC>,
    #[doc = "0x14 - Slice Timer Control"]
    pub tc: crate::Reg<tc::TC_SPEC>,
    #[doc = "0x18 - Passive Level Config"]
    pub psl: crate::Reg<psl::PSL_SPEC>,
    #[doc = "0x1c - Dither Config"]
    pub dit: crate::Reg<dit::DIT_SPEC>,
    #[doc = "0x20 - Dither Shadow Register"]
    pub dits: crate::Reg<dits::DITS_SPEC>,
    #[doc = "0x24 - Prescaler Control"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x28 - Floating Prescaler Control"]
    pub fpc: crate::Reg<fpc::FPC_SPEC>,
    #[doc = "0x2c - Floating Prescaler Shadow"]
    pub fpcs: crate::Reg<fpcs::FPCS_SPEC>,
    #[doc = "0x30 - Timer Period Value"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x34 - Timer Shadow Period Value"]
    pub prs: crate::Reg<prs::PRS_SPEC>,
    #[doc = "0x38 - Timer Compare Value"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x3c - Timer Shadow Compare Value"]
    pub crs: crate::Reg<crs::CRS_SPEC>,
    _reserved16: [u8; 0x30],
    #[doc = "0x70 - Timer Value"]
    pub timer: crate::Reg<timer::TIMER_SPEC>,
    #[doc = "0x74 - Capture Register 0"]
    pub c0v: crate::Reg<c0v::C0V_SPEC>,
    #[doc = "0x78 - Capture Register 1"]
    pub c1v: crate::Reg<c1v::C1V_SPEC>,
    #[doc = "0x7c - Capture Register 2"]
    pub c2v: crate::Reg<c2v::C2V_SPEC>,
    #[doc = "0x80 - Capture Register 3"]
    pub c3v: crate::Reg<c3v::C3V_SPEC>,
    _reserved21: [u8; 0x1c],
    #[doc = "0xa0 - Interrupt Status"]
    pub ints: crate::Reg<ints::INTS_SPEC>,
    #[doc = "0xa4 - Interrupt Enable Control"]
    pub inte: crate::Reg<inte::INTE_SPEC>,
    #[doc = "0xa8 - Service Request Selector"]
    pub srs: crate::Reg<srs::SRS_SPEC>,
    #[doc = "0xac - Interrupt Status Set"]
    pub sws: crate::Reg<sws::SWS_SPEC>,
    #[doc = "0xb0 - Interrupt Status Clear"]
    pub swr: crate::Reg<swr::SWR_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xb8 - Extended Read Back 0"]
    pub ecrd0: crate::Reg<ecrd0::ECRD0_SPEC>,
    #[doc = "0xbc - Extended Read Back 1"]
    pub ecrd1: crate::Reg<ecrd1::ECRD1_SPEC>,
}
#[doc = "INS register accessor: an alias for `Reg<INS_SPEC>`"]
pub type INS = crate::Reg<ins::INS_SPEC>;
#[doc = "Input Selector Configuration"]
pub mod ins;
#[doc = "CMC register accessor: an alias for `Reg<CMC_SPEC>`"]
pub type CMC = crate::Reg<cmc::CMC_SPEC>;
#[doc = "Connection Matrix Control"]
pub mod cmc;
#[doc = "TCST register accessor: an alias for `Reg<TCST_SPEC>`"]
pub type TCST = crate::Reg<tcst::TCST_SPEC>;
#[doc = "Slice Timer Status"]
pub mod tcst;
#[doc = "TCSET register accessor: an alias for `Reg<TCSET_SPEC>`"]
pub type TCSET = crate::Reg<tcset::TCSET_SPEC>;
#[doc = "Slice Timer Run Set"]
pub mod tcset;
#[doc = "TCCLR register accessor: an alias for `Reg<TCCLR_SPEC>`"]
pub type TCCLR = crate::Reg<tcclr::TCCLR_SPEC>;
#[doc = "Slice Timer Clear"]
pub mod tcclr;
#[doc = "TC register accessor: an alias for `Reg<TC_SPEC>`"]
pub type TC = crate::Reg<tc::TC_SPEC>;
#[doc = "Slice Timer Control"]
pub mod tc;
#[doc = "PSL register accessor: an alias for `Reg<PSL_SPEC>`"]
pub type PSL = crate::Reg<psl::PSL_SPEC>;
#[doc = "Passive Level Config"]
pub mod psl;
#[doc = "DIT register accessor: an alias for `Reg<DIT_SPEC>`"]
pub type DIT = crate::Reg<dit::DIT_SPEC>;
#[doc = "Dither Config"]
pub mod dit;
#[doc = "DITS register accessor: an alias for `Reg<DITS_SPEC>`"]
pub type DITS = crate::Reg<dits::DITS_SPEC>;
#[doc = "Dither Shadow Register"]
pub mod dits;
#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "Prescaler Control"]
pub mod psc;
#[doc = "FPC register accessor: an alias for `Reg<FPC_SPEC>`"]
pub type FPC = crate::Reg<fpc::FPC_SPEC>;
#[doc = "Floating Prescaler Control"]
pub mod fpc;
#[doc = "FPCS register accessor: an alias for `Reg<FPCS_SPEC>`"]
pub type FPCS = crate::Reg<fpcs::FPCS_SPEC>;
#[doc = "Floating Prescaler Shadow"]
pub mod fpcs;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Timer Period Value"]
pub mod pr;
#[doc = "PRS register accessor: an alias for `Reg<PRS_SPEC>`"]
pub type PRS = crate::Reg<prs::PRS_SPEC>;
#[doc = "Timer Shadow Period Value"]
pub mod prs;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Timer Compare Value"]
pub mod cr;
#[doc = "CRS register accessor: an alias for `Reg<CRS_SPEC>`"]
pub type CRS = crate::Reg<crs::CRS_SPEC>;
#[doc = "Timer Shadow Compare Value"]
pub mod crs;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Timer Value"]
pub mod timer;
#[doc = "C0V register accessor: an alias for `Reg<C0V_SPEC>`"]
pub type C0V = crate::Reg<c0v::C0V_SPEC>;
#[doc = "Capture Register 0"]
pub mod c0v;
#[doc = "C1V register accessor: an alias for `Reg<C1V_SPEC>`"]
pub type C1V = crate::Reg<c1v::C1V_SPEC>;
#[doc = "Capture Register 1"]
pub mod c1v;
#[doc = "C2V register accessor: an alias for `Reg<C2V_SPEC>`"]
pub type C2V = crate::Reg<c2v::C2V_SPEC>;
#[doc = "Capture Register 2"]
pub mod c2v;
#[doc = "C3V register accessor: an alias for `Reg<C3V_SPEC>`"]
pub type C3V = crate::Reg<c3v::C3V_SPEC>;
#[doc = "Capture Register 3"]
pub mod c3v;
#[doc = "INTS register accessor: an alias for `Reg<INTS_SPEC>`"]
pub type INTS = crate::Reg<ints::INTS_SPEC>;
#[doc = "Interrupt Status"]
pub mod ints;
#[doc = "INTE register accessor: an alias for `Reg<INTE_SPEC>`"]
pub type INTE = crate::Reg<inte::INTE_SPEC>;
#[doc = "Interrupt Enable Control"]
pub mod inte;
#[doc = "SRS register accessor: an alias for `Reg<SRS_SPEC>`"]
pub type SRS = crate::Reg<srs::SRS_SPEC>;
#[doc = "Service Request Selector"]
pub mod srs;
#[doc = "SWS register accessor: an alias for `Reg<SWS_SPEC>`"]
pub type SWS = crate::Reg<sws::SWS_SPEC>;
#[doc = "Interrupt Status Set"]
pub mod sws;
#[doc = "SWR register accessor: an alias for `Reg<SWR_SPEC>`"]
pub type SWR = crate::Reg<swr::SWR_SPEC>;
#[doc = "Interrupt Status Clear"]
pub mod swr;
#[doc = "ECRD0 register accessor: an alias for `Reg<ECRD0_SPEC>`"]
pub type ECRD0 = crate::Reg<ecrd0::ECRD0_SPEC>;
#[doc = "Extended Read Back 0"]
pub mod ecrd0;
#[doc = "ECRD1 register accessor: an alias for `Reg<ECRD1_SPEC>`"]
pub type ECRD1 = crate::Reg<ecrd1::ECRD1_SPEC>;
#[doc = "Extended Read Back 1"]
pub mod ecrd1;
