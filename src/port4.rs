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
#[doc = "OUT (rw) register accessor: Port 4 Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port 4 Output Register"]
pub mod out;
#[doc = "OMR (w) register accessor: Port 4 Output Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`omr`]
module"]
pub type OMR = crate::Reg<omr::OMR_SPEC>;
#[doc = "Port 4 Output Modification Register"]
pub mod omr;
#[doc = "IOCR0 (rw) register accessor: Port 4 Input/Output Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iocr0`]
module"]
pub type IOCR0 = crate::Reg<iocr0::IOCR0_SPEC>;
#[doc = "Port 4 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "IN (r) register accessor: Port 4 Input Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port 4 Input Register"]
pub mod in_;
#[doc = "PDR0 (rw) register accessor: Port 4 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdr0`]
module"]
pub type PDR0 = crate::Reg<pdr0::PDR0_SPEC>;
#[doc = "Port 4 Pad Driver Mode 0 Register"]
pub mod pdr0;
#[doc = "PDISC (r) register accessor: Port 4 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdisc`]
module"]
pub type PDISC = crate::Reg<pdisc::PDISC_SPEC>;
#[doc = "Port 4 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "PPS (rw) register accessor: Port 4 Pin Power Save Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pps`]
module"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "Port 4 Pin Power Save Register"]
pub mod pps;
#[doc = "HWSEL (rw) register accessor: Port 4 Pin Hardware Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hwsel`]
module"]
pub type HWSEL = crate::Reg<hwsel::HWSEL_SPEC>;
#[doc = "Port 4 Pin Hardware Select Register"]
pub mod hwsel;
