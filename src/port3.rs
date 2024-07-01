#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    out: OUT,
    omr: OMR,
    _reserved2: [u8; 0x08],
    iocr0: IOCR0,
    iocr4: IOCR4,
    _reserved4: [u8; 0x0c],
    in_: IN,
    _reserved5: [u8; 0x18],
    pdr0: PDR0,
    _reserved6: [u8; 0x1c],
    pdisc: PDISC,
    _reserved7: [u8; 0x0c],
    pps: PPS,
    hwsel: HWSEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Output Register"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x04 - Port 3 Output Modification Register"]
    #[inline(always)]
    pub const fn omr(&self) -> &OMR {
        &self.omr
    }
    #[doc = "0x10 - Port 3 Input/Output Control Register 0"]
    #[inline(always)]
    pub const fn iocr0(&self) -> &IOCR0 {
        &self.iocr0
    }
    #[doc = "0x14 - Port 3 Input/Output Control Register 4"]
    #[inline(always)]
    pub const fn iocr4(&self) -> &IOCR4 {
        &self.iocr4
    }
    #[doc = "0x24 - Port 3 Input Register"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x40 - Port 3 Pad Driver Mode 0 Register"]
    #[inline(always)]
    pub const fn pdr0(&self) -> &PDR0 {
        &self.pdr0
    }
    #[doc = "0x60 - Port 3 Pin Function Decision Control Register"]
    #[inline(always)]
    pub const fn pdisc(&self) -> &PDISC {
        &self.pdisc
    }
    #[doc = "0x70 - Port 3 Pin Power Save Register"]
    #[inline(always)]
    pub const fn pps(&self) -> &PPS {
        &self.pps
    }
    #[doc = "0x74 - Port 3 Pin Hardware Select Register"]
    #[inline(always)]
    pub const fn hwsel(&self) -> &HWSEL {
        &self.hwsel
    }
}
#[doc = "OUT (rw) register accessor: Port 3 Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port 3 Output Register"]
pub mod out;
#[doc = "OMR (w) register accessor: Port 3 Output Modification Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omr`]
module"]
pub type OMR = crate::Reg<omr::OMR_SPEC>;
#[doc = "Port 3 Output Modification Register"]
pub mod omr;
#[doc = "IOCR0 (rw) register accessor: Port 3 Input/Output Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`iocr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocr0`]
module"]
pub type IOCR0 = crate::Reg<iocr0::IOCR0_SPEC>;
#[doc = "Port 3 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "IOCR4 (rw) register accessor: Port 3 Input/Output Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`iocr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocr4`]
module"]
pub type IOCR4 = crate::Reg<iocr4::IOCR4_SPEC>;
#[doc = "Port 3 Input/Output Control Register 4"]
pub mod iocr4;
#[doc = "IN (r) register accessor: Port 3 Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port 3 Input Register"]
pub mod in_;
#[doc = "PDR0 (rw) register accessor: Port 3 Pad Driver Mode 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdr0`]
module"]
pub type PDR0 = crate::Reg<pdr0::PDR0_SPEC>;
#[doc = "Port 3 Pad Driver Mode 0 Register"]
pub mod pdr0;
#[doc = "PDISC (r) register accessor: Port 3 Pin Function Decision Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdisc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdisc`]
module"]
pub type PDISC = crate::Reg<pdisc::PDISC_SPEC>;
#[doc = "Port 3 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "PPS (rw) register accessor: Port 3 Pin Power Save Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps`]
module"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "Port 3 Pin Power Save Register"]
pub mod pps;
#[doc = "HWSEL (rw) register accessor: Port 3 Pin Hardware Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwsel`]
module"]
pub type HWSEL = crate::Reg<hwsel::HWSEL_SPEC>;
#[doc = "Port 3 Pin Hardware Select Register"]
pub mod hwsel;
