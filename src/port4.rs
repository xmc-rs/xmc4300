#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 4 Output Register"]
    pub out: OUT,
    #[doc = "0x04 - Port 4 Output Modification Register"]
    pub omr: OMR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Port 4 Input/Output Control Register 0"]
    pub iocr0: IOCR0,
    _reserved3: [u8; 0x10],
    #[doc = "0x24 - Port 4 Input Register"]
    pub in_: IN,
    _reserved4: [u8; 0x18],
    #[doc = "0x40 - Port 4 Pad Driver Mode 0 Register"]
    pub pdr0: PDR0,
    _reserved5: [u8; 0x1c],
    #[doc = "0x60 - Port 4 Pin Function Decision Control Register"]
    pub pdisc: PDISC,
    _reserved6: [u8; 0x0c],
    #[doc = "0x70 - Port 4 Pin Power Save Register"]
    pub pps: PPS,
    #[doc = "0x74 - Port 4 Pin Hardware Select Register"]
    pub hwsel: HWSEL,
}
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port 4 Output Register"]
pub mod out;
#[doc = "OMR (w) register accessor: an alias for `Reg<OMR_SPEC>`"]
pub type OMR = crate::Reg<omr::OMR_SPEC>;
#[doc = "Port 4 Output Modification Register"]
pub mod omr;
#[doc = "IOCR0 (rw) register accessor: an alias for `Reg<IOCR0_SPEC>`"]
pub type IOCR0 = crate::Reg<iocr0::IOCR0_SPEC>;
#[doc = "Port 4 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port 4 Input Register"]
pub mod in_;
#[doc = "PDR0 (rw) register accessor: an alias for `Reg<PDR0_SPEC>`"]
pub type PDR0 = crate::Reg<pdr0::PDR0_SPEC>;
#[doc = "Port 4 Pad Driver Mode 0 Register"]
pub mod pdr0;
#[doc = "PDISC (r) register accessor: an alias for `Reg<PDISC_SPEC>`"]
pub type PDISC = crate::Reg<pdisc::PDISC_SPEC>;
#[doc = "Port 4 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "PPS (rw) register accessor: an alias for `Reg<PPS_SPEC>`"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "Port 4 Pin Power Save Register"]
pub mod pps;
#[doc = "HWSEL (rw) register accessor: an alias for `Reg<HWSEL_SPEC>`"]
pub type HWSEL = crate::Reg<hwsel::HWSEL_SPEC>;
#[doc = "Port 4 Pin Hardware Select Register"]
pub mod hwsel;
