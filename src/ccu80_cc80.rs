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
    #[doc = "0x38 - Channel 1 Compare Value"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x3c - Channel 1 Compare Shadow Value"]
    pub cr1s: crate::Reg<cr1s::CR1S_SPEC>,
    #[doc = "0x40 - Channel 2 Compare Value"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x44 - Channel 2 Compare Shadow Value"]
    pub cr2s: crate::Reg<cr2s::CR2S_SPEC>,
    #[doc = "0x48 - Channel Control"]
    pub chc: crate::Reg<chc::CHC_SPEC>,
    #[doc = "0x4c - Dead Time Control"]
    pub dtc: crate::Reg<dtc::DTC_SPEC>,
    #[doc = "0x50 - Channel 1 Dead Time Values"]
    pub dc1r: crate::Reg<dc1r::DC1R_SPEC>,
    #[doc = "0x54 - Channel 2 Dead Time Values"]
    pub dc2r: crate::Reg<dc2r::DC2R_SPEC>,
    _reserved22: [u8; 0x18],
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
    _reserved27: [u8; 0x1c],
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
    #[doc = "0xb4 - Shadow transfer control"]
    pub stc: crate::Reg<stc::STC_SPEC>,
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
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Channel 1 Compare Value"]
pub mod cr1;
#[doc = "CR1S register accessor: an alias for `Reg<CR1S_SPEC>`"]
pub type CR1S = crate::Reg<cr1s::CR1S_SPEC>;
#[doc = "Channel 1 Compare Shadow Value"]
pub mod cr1s;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Channel 2 Compare Value"]
pub mod cr2;
#[doc = "CR2S register accessor: an alias for `Reg<CR2S_SPEC>`"]
pub type CR2S = crate::Reg<cr2s::CR2S_SPEC>;
#[doc = "Channel 2 Compare Shadow Value"]
pub mod cr2s;
#[doc = "CHC register accessor: an alias for `Reg<CHC_SPEC>`"]
pub type CHC = crate::Reg<chc::CHC_SPEC>;
#[doc = "Channel Control"]
pub mod chc;
#[doc = "DTC register accessor: an alias for `Reg<DTC_SPEC>`"]
pub type DTC = crate::Reg<dtc::DTC_SPEC>;
#[doc = "Dead Time Control"]
pub mod dtc;
#[doc = "DC1R register accessor: an alias for `Reg<DC1R_SPEC>`"]
pub type DC1R = crate::Reg<dc1r::DC1R_SPEC>;
#[doc = "Channel 1 Dead Time Values"]
pub mod dc1r;
#[doc = "DC2R register accessor: an alias for `Reg<DC2R_SPEC>`"]
pub type DC2R = crate::Reg<dc2r::DC2R_SPEC>;
#[doc = "Channel 2 Dead Time Values"]
pub mod dc2r;
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
#[doc = "STC register accessor: an alias for `Reg<STC_SPEC>`"]
pub type STC = crate::Reg<stc::STC_SPEC>;
#[doc = "Shadow transfer control"]
pub mod stc;
#[doc = "ECRD0 register accessor: an alias for `Reg<ECRD0_SPEC>`"]
pub type ECRD0 = crate::Reg<ecrd0::ECRD0_SPEC>;
#[doc = "Extended Read Back 0"]
pub mod ecrd0;
#[doc = "ECRD1 register accessor: an alias for `Reg<ECRD1_SPEC>`"]
pub type ECRD1 = crate::Reg<ecrd1::ECRD1_SPEC>;
#[doc = "Extended Read Back 1"]
pub mod ecrd1;
